use std::fs;

const FILEPATH: &str = "src/day5/input.txt";

#[derive(Debug)]
struct GardenMap {
    source: &'static str,
    destination: &'static str,
    map: Vec<(usize, usize, usize)>,
}

impl GardenMap {
    fn new(source: &'static str, destination: &'static str) -> GardenMap {
        return GardenMap { source: source, destination: destination, map: vec![] };
    }

    fn add_range(&mut self, source: usize, length: usize, dest: usize) {
        self.map.push((source, length, dest));
        // print!("{:?}", self.map);
    }

    fn load_data(&mut self, almanac: &Vec<&str>) {
        let mut file_index: usize = 0;
        let key = format!("{}-to-{} map:", self.source, self.destination);
        while file_index < almanac.len() && almanac[file_index] != &key {
            file_index = file_index + 1;
        }
        if file_index == almanac.len() {
            panic!("Map not found for map from {} to {}", self.source, self.destination);
        }
        file_index = file_index + 1;
        while file_index < almanac.len() && almanac[file_index] != "" {
            let range_description: Vec<usize> = almanac[file_index]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            self.add_range(range_description[1], range_description[2], range_description[0]);
            file_index = file_index + 1;
        }
        self.sort();
    }

    fn get_dest(&self, source_index: usize) -> usize {
        for (source, length, dest) in &self.map {
            if source_index >= *source && source_index < *source + *length {
                return dest + source_index - source;
            }
        }
        return source_index;
    }

    fn sort(&mut self) {
        self.map.sort_by(|a,b| a.0.cmp(&b.0));
    }

    fn get_image_interval(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut return_list: Vec<(usize, usize)> = vec![];

        let mut input_queue: Vec<(usize,usize)> = vec![];
        input_queue.push((x,y));

        while input_queue.len() > 0 {
            println!("Queue is: {:?}", input_queue);

            let (lbound,ubound) = input_queue.pop().unwrap();
            println!("Checking {} - {} -=-=-=-=-=-=-=-", &lbound, &ubound);
            let mut found = false;
            for (source, length, dest) in &self.map {
                println!("Comparing with {}-{}", *source, *source+ *length);
                if ubound <= *source || lbound >= *source + *length {                    
                    println!("disjoint");
                    continue;
                }
                found = true;

                // Compute the boundaries of the intersection interval
                let inter_lbound = lbound.max(*source);
                let inter_ubound = ubound.min(*source + *length);
                return_list.push((*dest + inter_lbound - *source, *dest + inter_ubound - *source));

                if lbound < *source {
                    println!("Something is out below - pushing {}-{}", &lbound, *source);
                    input_queue.push((lbound, *source));
                }
                
                if ubound > *source + *length {
                    println!("Something is out above - pushing {}-{}", *source + *length, ubound);
                    input_queue.push((*source + *length, ubound));
                }
                break;
            }

            if !found {
                // the interval is outside of any modifier range - dest = source
                return_list.push((lbound, ubound));
            }

        }
        return return_list;
    }

    fn get_all_image_intervals(
        &self,
        source_intervals: &Vec<(usize, usize)>
    ) -> Vec<(usize, usize)> {
        let mut dest_intervals: Vec<(usize, usize)> = vec![];
        for (s_lbound, s_ubound) in source_intervals {
            let interval_sublist = self.get_image_interval(*s_lbound, *s_ubound);
            interval_sublist.iter().for_each(|(a, b)| dest_intervals.push((*a, *b)));
        }
        return dest_intervals;
    }
}

#[test]
fn test_garden_map() {
    let mut my_map = GardenMap::new("seed", "soil");

    my_map.add_range(98, 2, 50);
    my_map.add_range(50, 48, 52);
    assert_eq!(my_map.get_dest(0), 0);
    assert_eq!(my_map.get_dest(1), 1);
    assert_eq!(my_map.get_dest(30), 30);
    assert_eq!(my_map.get_dest(23), 23);
    assert_eq!(my_map.get_dest(49), 49);
    assert_eq!(my_map.get_dest(50), 52);
    assert_eq!(my_map.get_dest(51), 53);
    assert_eq!(my_map.get_dest(52), 54);
    assert_eq!(my_map.get_dest(97), 99);
    assert_eq!(my_map.get_dest(98), 50);
    assert_eq!(my_map.get_dest(99), 51);
    assert_eq!(my_map.get_dest(100), 100);
}

#[test]
fn test_garden_map_intervals() {

    let mut my_map = GardenMap::new("seed", "soil");

    my_map.add_range(80, 20, 50);
    my_map.add_range(50, 20, 80);

    // let first_int = my_map.get_image_interval(1, 10);
    // assert_eq!(first_int.len(),1);    
    // assert_eq!(first_int[0],(1,10));

    // let second_int = my_map.get_image_interval(55, 60);
    // assert_eq!(second_int.len(),1);    
    // assert_eq!(second_int[0],(85,90));

    // let third_int = my_map.get_image_interval(45, 60);
    // assert_eq!(third_int.len(),2);    
    // assert_eq!(third_int[1],(45,50));
    // assert_eq!(third_int[0],(80,90));
    
    let mut fourth_int = my_map.get_image_interval(45, 75);
    assert_eq!(fourth_int.len(),3);    
    fourth_int.sort_by(|a,b| a.0.cmp(&b.0));
    println!("{:?}",&fourth_int);    
    assert_eq!(fourth_int[0],(45,50));
    assert_eq!(fourth_int[1],(70,75));
    assert_eq!(fourth_int[2],(80,100));
    

}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let (_, seeds_string) = line_array[0].split_at("seeds: ".len());
    let seed_list: Vec<usize> = seeds_string
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut seed_soil_map = GardenMap::new("seed", "soil");
    let mut soil_fertilizer_map = GardenMap::new("soil", "fertilizer");
    let mut fertilizer_water_map = GardenMap::new("fertilizer", "water");
    let mut water_light_map = GardenMap::new("water", "light");
    let mut light_temp_map = GardenMap::new("light", "temperature");
    let mut temp_humidity_map = GardenMap::new("temperature", "humidity");
    let mut humidity_location_map = GardenMap::new("humidity", "location");

    seed_soil_map.load_data(&line_array);
    soil_fertilizer_map.load_data(&line_array);
    fertilizer_water_map.load_data(&line_array);
    water_light_map.load_data(&line_array);
    light_temp_map.load_data(&line_array);
    temp_humidity_map.load_data(&line_array);
    humidity_location_map.load_data(&line_array);

    let mut list_intervals: Vec<(usize, usize)> = vec![];
    let mut seed_index: usize = 0;
    while seed_index < seed_list.len() {
        list_intervals.push((
            seed_list[seed_index],
            seed_list[seed_index] + seed_list[seed_index + 1],
        ));
        seed_index = seed_index + 2;
    }

    println!("Seed intervals: {:?}", &list_intervals);
    let soil_intervals = seed_soil_map.get_all_image_intervals(&list_intervals);
    println!("soil intervals: {:?}", &soil_intervals);
    let fert_intervals = soil_fertilizer_map.get_all_image_intervals(&soil_intervals);
    println!("fert intervals: {:?}", &fert_intervals);
    let water_intervals = fertilizer_water_map.get_all_image_intervals(&fert_intervals);
    println!("water intervals: {:?}", &water_intervals);
    let light_intervals = water_light_map.get_all_image_intervals(&water_intervals);
    println!("light intervals: {:?}", &light_intervals);
    let temp_intervals = light_temp_map.get_all_image_intervals(&light_intervals);
    println!("temp intervals: {:?}", &temp_intervals);
    let hum_intervals = temp_humidity_map.get_all_image_intervals(&temp_intervals);
    println!("humidity intervals: {:?}", &hum_intervals);
    let loc_intervals = humidity_location_map.get_all_image_intervals(&hum_intervals);
    println!("Location intervals: {:?}", &loc_intervals);

    println!(
        "The minimum is: {}",
        loc_intervals.iter().fold(usize::MAX, |accumul, (a, _b)| accumul.min(*a))
    );
}
