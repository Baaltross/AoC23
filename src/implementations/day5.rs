use std::str::FromStr;
use strum::{EnumCount, EnumIter, EnumString};

use crate::utility::generic_error::{GenericError, GenericResult};

#[derive(EnumCount, EnumIter, Copy, Clone, PartialEq, EnumString)]
enum EntityType {
    #[strum(serialize = "seed")]
    Seed,
    #[strum(serialize = "soil")]
    Soil,
    #[strum(serialize = "fertilizer")]
    Fertiliser,
    #[strum(serialize = "water")]
    Water,
    #[strum(serialize = "light")]
    Light,
    #[strum(serialize = "temperature")]
    Temperature,
    #[strum(serialize = "humidity")]
    Humidity,
    #[strum(serialize = "location")]
    Location,
}

struct Mapping {
    source: usize,      // start index of source
    destination: usize, // start index of destination
    span: usize,        // number of entries across both source and destination
}

impl Mapping {
    fn source_begin(&self) -> usize {
        self.source
    }

    fn source_end(&self) -> usize {
        self.source + self.span
    }

    fn destination_begin(&self) -> usize {
        self.destination
    }

    fn destination_end(&self) -> usize {
        self.destination + self.span
    }

    fn do_mapping(&self, source: usize) -> usize {
        source - self.source + self.destination
    }
}

impl FromStr for Mapping {
    type Err = GenericError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parsed = sscanf::sscanf!(input, "{} {} {}", usize, usize, usize)?;
        Ok(Mapping {
            source: parsed.1,
            destination: parsed.0,
            span: parsed.2,
        })
    }
}

struct MappingGroup {
    source_type: EntityType,
    destination_type: EntityType,
    mappings: Vec<Mapping>,
}

impl MappingGroup {
    fn do_mapping(&self, input: usize) -> usize {
        for mapping in self.mappings.iter() {
            if input >= mapping.source && input < mapping.source + mapping.span {
                return mapping.do_mapping(input);
            }
        }
        input
    }

    fn do_range_mapping(&self, input: &std::ops::Range<usize>) -> Vec<std::ops::Range<usize>> {
        let mut unmapped: Vec<std::ops::Range<usize>> = vec![input.clone()];
        let mut output: Vec<std::ops::Range<usize>> = vec![];

        for mapping in self.mappings.iter() {
            let mut new_unmapped: Vec<std::ops::Range<usize>> = vec![];

            for test_range in unmapped {
                // Note the unusual combination of >= and < here is to deal with start being inclusive and end being exclusive
                if mapping.source_end() >= test_range.start
                    && mapping.source_begin() < test_range.end
                {
                    if mapping.source_begin() >= test_range.start
                        && mapping.source_end() <= test_range.end
                    {
                        // mapping is fully contained within test_range
                        new_unmapped.push(test_range.start..mapping.source_begin()); // unmapped
                        output.push(mapping.destination_begin()..mapping.destination_end()); // mapped
                        new_unmapped.push(mapping.source_end()..test_range.end); // unmapped
                    } else if mapping.source_begin() <= test_range.start
                        && mapping.source_end() >= test_range.end
                    {
                        // test_range is fully contained within mapping
                        output.push(mapping.do_mapping(test_range.start)..mapping.do_mapping(test_range.end),); // mapped
                    } else if mapping.source_begin() <= test_range.start {
                        // Mapping overlaps start of test_range
                        output.push(mapping.do_mapping(test_range.start)..mapping.destination_end()); // mapped
                        new_unmapped.push(mapping.source_end()..test_range.end); // unmapped
                    } else {
                        // Mapping overlaps end of test_range
                        new_unmapped.push(test_range.start..mapping.source_begin()); // unmapped
                        output.push(mapping.destination_begin()..mapping.do_mapping(test_range.end)); // mapped
                    }
                } else {
                    // No overlap
                    new_unmapped.push(test_range.clone());
                }
            }

            // for check_value in new_output.iter() {
            //     assert!(check_value.start != check_value.end);
            // }

            new_unmapped.retain(|check_value| check_value.start != check_value.end);
            unmapped = new_unmapped;
        }

        output.append(&mut unmapped);
        output.retain(|check_value| check_value.start != check_value.end);
        output
    }
}

impl FromStr for MappingGroup {
    type Err = GenericError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut source_type: Option<EntityType> = None;
        let mut destination_type: Option<EntityType> = None;
        let mut mappings: Vec<Mapping> = vec![];

        for (index, line) in input.lines().enumerate() {
            if index == 0 {
                let map_types = sscanf::scanf!(line, "{}-to-{} map:", str, str)?;
                source_type = Some(EntityType::from_str(map_types.0)?);
                destination_type = Some(EntityType::from_str(map_types.1)?);
            } else {
                mappings.push(Mapping::from_str(line)?);
            }
        }

        if source_type.is_none() || destination_type.is_none() {
            return Err(GenericError::BasicError(String::from(
                "Failed to get source/destination type",
            )));
        }

        Ok(MappingGroup {
            source_type: source_type.unwrap(),
            destination_type: destination_type.unwrap(),
            mappings: mappings,
        })
    }
}

struct ProblemSet {
    mapping_groups: Vec<MappingGroup>,
    seeds: Vec<usize>,
}

impl ProblemSet {
    fn get_location_for_seed(&self, seed: usize) -> usize {
        let mut result: usize = seed;
        for mapping_group in self.mapping_groups.iter() {
            result = mapping_group.do_mapping(result);
        }
        result
    }

    fn get_min_location_for_seed_range(&self, first_seed: usize, seed_count: usize) -> usize {
        let mut ranges: Vec<std::ops::Range<usize>> = vec![first_seed..(first_seed + seed_count)];
        for mapping_group in self.mapping_groups.iter() {
            let mut new_ranges: Vec<std::ops::Range<usize>> = vec![];

            for range in ranges {
                new_ranges.append(&mut mapping_group.do_range_mapping(&range));
            }

            ranges = new_ranges;
        }
        ranges
            .iter()
            .fold(usize::MAX, |acc, e| std::cmp::min(acc, e.start))
    }
}

impl FromStr for ProblemSet {
    type Err = GenericError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut output = ProblemSet {
            mapping_groups: vec![],
            seeds: vec![],
        };
        output.mapping_groups.reserve(EntityType::COUNT - 1);

        let replaced_input = input.replace("\r\n", "\n");
        let file_sections: Vec<&str> = replaced_input.split("\n\n").collect();

        // Parse seeds
        {
            let split_str: Vec<&str> = file_sections[0].split_whitespace().collect();
            assert!(split_str[0] == "seeds:");

            for seed_str in &split_str[1..] {
                output.seeds.push(seed_str.parse()?);
            }
        }

        // Parse mapping groups
        for section in &file_sections[1..] {
            output.mapping_groups.push(MappingGroup::from_str(section)?);
        }

        {
            // Validate that the sections we have retrieved are in the correct order
            for (index, mapping_group) in output.mapping_groups.iter().enumerate() {
                assert!(mapping_group.source_type as usize == index);
                assert!(mapping_group.destination_type as usize == index + 1);
            }
        }

        Ok(output)
    }
}

fn part_1(input_path: &str) -> GenericResult<usize> {
    let file_contents = std::fs::read_to_string(input_path)?;

    let mut result = usize::MAX;

    let problem_set = ProblemSet::from_str(file_contents.as_str())?;

    for seed in problem_set.seeds.iter() {
        result = std::cmp::min(result, problem_set.get_location_for_seed(*seed));
    }

    Ok(result)
}

fn part_2(input_path: &str) -> GenericResult<usize> {
    let file_contents = std::fs::read_to_string(input_path)?;

    let mut result = usize::MAX;

    let problem_set = ProblemSet::from_str(file_contents.as_str())?;

    for seed_group in problem_set.seeds.chunks(2) {
        result = std::cmp::min(
            result,
            problem_set.get_min_location_for_seed_range(seed_group[0], seed_group[1]),
        );
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("test_data/day5/example.txt")?, 35);
    Ok(())
}

#[test]
pub fn stress_test_mapping_group() -> GenericResult<()> {
    let mapping_group = MappingGroup {
        source_type: EntityType::Seed,
        destination_type: EntityType::Fertiliser,
        mappings: vec![Mapping {
            source: 10,
            destination: 30,
            span: 10,
        }],
    };

    // Single output
    assert_eq!(mapping_group.do_range_mapping(&(0..2)), vec!(0..2)); // Fully separate left of mapping
    assert_eq!(mapping_group.do_range_mapping(&(8..10)), vec!(8..10)); // Adjacent left of mapping
    assert_eq!(mapping_group.do_range_mapping(&(10..12)), vec!(30..32)); // Tight left of mapping
    assert_eq!(mapping_group.do_range_mapping(&(12..18)), vec!(32..38)); // Fully contained inside mapping
    assert_eq!(mapping_group.do_range_mapping(&(10..20)), vec!(30..40)); // Tightly contained inside mapping
    assert_eq!(mapping_group.do_range_mapping(&(18..20)), vec!(38..40)); // Tight right of mapping
    assert_eq!(mapping_group.do_range_mapping(&(20..22)), vec!(20..22)); // Adjacent right of mapping
    assert_eq!(mapping_group.do_range_mapping(&(22..24)), vec!(22..24)); // Fully separate right of mapping

    // Multi output
    assert_eq!(
        mapping_group.do_range_mapping(&(8..12)),
        vec!(30..32,8..10)
    ); // Intersection left of mapping
    assert_eq!(
        mapping_group.do_range_mapping(&(18..22)),
        vec!(38..40, 20..22)
    ); // Intersection right of mapping
    assert_eq!(
        mapping_group.do_range_mapping(&(8..22)),
        vec!(30..40, 8..10, 20..22)
    ); // Mapping fully contained within range
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("test_data/day5/example.txt").unwrap(), 46);
    Ok(())
}

pub fn run(input_path: &String) -> GenericResult<()> {
    println!("Part one result: {}", part_1(&input_path)?);
    println!("Part two result: {}", part_2(&input_path)?);
    Ok(())
}
