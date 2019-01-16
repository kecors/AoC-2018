use pom::parser::*;
use pom::Error;
use std::cmp::{Ordering, Reverse};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{stdin, Read};
use std::str;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

#[derive(Debug)]
struct Group {
    units: u32,
    hit_points: u32,
    attack_damage: u32,
    attack_type: AttackType,
    initiative: u32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.attack_damage
    }

    fn damage_multiplier(&self, attack_type: AttackType) -> u32 {
        if self.weaknesses.contains(&attack_type) {
            2
        } else if self.immunities.contains(&attack_type) {
            0
        } else {
            1
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GroupType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug)]
struct Engine {
    groups: HashMap<u32, (GroupType, Group)>,
}

impl Engine {
    fn new(immune_system: Vec<Group>, infection: Vec<Group>) -> Engine {
        let mut groups = HashMap::new();

        let mut group_id = 100;
        for group in immune_system {
            groups.insert(group_id, (GroupType::ImmuneSystem, group));
            group_id += 1;
        }

        group_id = 200;
        for group in infection {
            groups.insert(group_id, (GroupType::Infection, group));
            group_id += 1;
        }

        Engine { groups }
    }

    fn fight(&mut self) -> u32 {
        loop {
            // Combat ends once one army has lost all its units
            let immune_system_units: u32 = self
                .groups
                .values()
                .filter(|(group_type, _)| *group_type == GroupType::ImmuneSystem)
                .map(|(_, group)| group.units)
                .sum();
            let infection_units: u32 = self
                .groups
                .values()
                .filter(|(group_type, _)| *group_type == GroupType::Infection)
                .map(|(_, group)| group.units)
                .sum();
            if immune_system_units == 0 {
                return infection_units;
            }
            if infection_units == 0 {
                return immune_system_units;
            }

            // Target selection phase

            // Determine attacker order
            let mut attacker_keys: Vec<u32> = self
                .groups
                .keys()
                .filter(|key| self.groups[key].1.units > 0)
                .cloned()
                .collect();
            attacker_keys.sort_by(|a, b| {
                match self.groups[b]
                    .1
                    .effective_power()
                    .cmp(&self.groups[a].1.effective_power())
                {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => self.groups[b]
                        .1
                        .initiative
                        .cmp(&self.groups[a].1.initiative),
                    Ordering::Greater => Ordering::Greater,
                }
            });

            // Determine target for each attacker
            let mut target_attackers: HashMap<u32, u32> = HashMap::new();
            for attacker_key in attacker_keys.iter() {
                let (attacker_group_type, attacker) = &self.groups[attacker_key];
                let mut selections: Vec<Reverse<(u32, u32, u32, u32)>> = self
                    .groups
                    .iter()
                    .filter(|(_, (group_type, _))| *group_type != *attacker_group_type)
                    .filter(|(_, (_, group))| group.units > 0)
                    .map(|(key, (_, group))| {
                        Reverse((
                            group.damage_multiplier(attacker.attack_type),
                            group.effective_power(),
                            group.initiative,
                            *key,
                        ))
                    })
                    .collect();
                selections.sort();

                for Reverse(selection) in selections {
                    // If the attacker cannot deal any damage, it does not select a target
                    if selection.0 == 0 {
                        break;
                    }

                    match target_attackers.entry(selection.3) {
                        Entry::Vacant(v) => {
                            v.insert(*attacker_key);
                            break;
                        }
                        Entry::Occupied(_) => {
                            // A group can be the target for only one attacker
                            continue;
                        }
                    }
                }
            }

            // Make HashMap to look up target for each attacker
            let mut attacker_targets: HashMap<u32, u32> = HashMap::new();
            for (target, attacker) in target_attackers.iter() {
                if let Some(existing_target) = attacker_targets.insert(*attacker, *target) {
                    panic!("existing target {:?}", existing_target);
                }
            }

            // Attacking Phase

            // Groups attack in decreasing order of initiative
            attacker_keys.sort_by(|a, b| {
                self.groups[b]
                    .1
                    .initiative
                    .cmp(&self.groups[a].1.initiative)
            });

            for attacker_key in attacker_keys.iter() {
                let (effective_power, attack_type) =
                    if let Some((_, attacker)) = self.groups.get(&attacker_key) {
                        if attacker.units == 0 {
                            continue;
                        }
                        (attacker.effective_power(), attacker.attack_type)
                    } else {
                        continue;
                    };
                if let Some(target_key) = attacker_targets.get(&attacker_key) {
                    if let Some((_, target)) = self.groups.get_mut(&target_key) {
                        let damage = effective_power * target.damage_multiplier(attack_type);
                        let units_killed = damage / target.hit_points;
                        if units_killed > target.units {
                            target.units = 0;
                        } else {
                            target.units -= units_killed;
                        }
                    }
                }
            }
        }
    }
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn number<'a>() -> Parser<'a, u8, u32> {
    let number = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| u32::from_str_radix(s, 10))
}

fn units<'a>() -> Parser<'a, u8, u32> {
    number() - space() - seq(b"units each with") - space()
}

fn hit_points<'a>() -> Parser<'a, u8, u32> {
    number() - space() - seq(b"hit points") - space()
}

fn attack_type<'a>() -> Parser<'a, u8, AttackType> {
    use AttackType::*;

    seq(b"bludgeoning").map(|_| Bludgeoning)
        | seq(b"cold").map(|_| Cold)
        | seq(b"fire").map(|_| Fire)
        | seq(b"radiation").map(|_| Radiation)
        | seq(b"slashing").map(|_| Slashing)
}

fn attack_types<'a>() -> Parser<'a, u8, Vec<AttackType>> {
    (seq(b", ").opt() * attack_type())
        .repeat(1..)
        .map(|attack_types| attack_types)
}

fn weaknesses<'a>() -> Parser<'a, u8, Vec<AttackType>> {
    seq(b"weak to ") * attack_types().map(|weaknesses| weaknesses)
}

fn immunities<'a>() -> Parser<'a, u8, Vec<AttackType>> {
    seq(b"immune to ") * attack_types().map(|weaknesses| weaknesses)
}

fn weaknesses_and_immunities<'a>() -> Parser<'a, u8, (Vec<AttackType>, Vec<AttackType>)> {
    (sym(b'(') * weaknesses() + seq(b"; ") * immunities() - sym(b')'))
        .map(|(weaknesses, immunities)| (weaknesses, immunities))
        | (sym(b'(') * immunities() + seq(b"; ") * weaknesses() - sym(b')'))
            .map(|(immunities, weaknesses)| (weaknesses, immunities))
        | (sym(b'(') * weaknesses() - sym(b')')).map(|weaknesses| (weaknesses, vec![]))
        | (sym(b'(') * immunities() - sym(b')')).map(|immunities| (vec![], immunities))
}

fn attack_damage<'a>() -> Parser<'a, u8, u32> {
    space() * seq(b"with an attack that does") * space() * number()
}

fn initiative<'a>() -> Parser<'a, u8, u32> {
    space() * seq(b"damage at initiative") * space() * number()
}

fn group<'a>() -> Parser<'a, u8, Group> {
    (units()
        + (hit_points()
            + (weaknesses_and_immunities()
                + (attack_damage() + (space() * attack_type() + initiative())))))
    .map(
        |(
            units,
            (hit_points, ((weaknesses, immunities), (attack_damage, (attack_type, initiative)))),
        )| Group {
            units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        },
    ) | (units() + (hit_points() + (attack_damage() + (space() * attack_type() + initiative()))))
        .map(
            |(units, (hit_points, (attack_damage, (attack_type, initiative))))| Group {
                units,
                hit_points,
                attack_damage,
                attack_type,
                initiative,
                weaknesses: vec![],
                immunities: vec![],
            },
        )
}

fn immune_system<'a>() -> Parser<'a, u8, Vec<Group>> {
    seq(b"Immune System:") * (space() * group()).repeat(1..)
}

fn infection<'a>() -> Parser<'a, u8, Vec<Group>> {
    space() * seq(b"Infection:") * (space() * group()).repeat(1..)
}

fn engine<'a>() -> Parser<'a, u8, Engine> {
    (immune_system() + infection())
        .map(|(immune_system, infection)| Engine::new(immune_system, infection))
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut engine = engine().parse(input.as_bytes())?;

    let winning_army_units = engine.fight();
    println!("Part 1: the winning army has {} units", winning_army_units);

    Ok(())
}
