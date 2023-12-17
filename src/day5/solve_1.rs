use std::fs;

#[derive(Debug)]
struct GardenMap {
    source: &'static str,
    destination: &'static str,
    map: Vec<(usize,usize,usize)>,
}

impl GardenMap {
    fn new(source: &'static str, destination: &'static str) -> GardenMap {
        return GardenMap { source: source, destination: destination, map: vec![] }
    }

    fn add_range(&mut self, source: usize, length: usize, dest: usize) {
        self.map.push((source,length,dest));
        // print!("{:?}", self.map);
    }

    fn load_data(&mut self, almanac: &Vec<&str>) {

        let mut file_index: usize = 0;
        let key = format!("{}-to-{} map:", self.source, self.destination);
        while file_index < almanac.len() && almanac[file_index] != &key {
            file_index = file_index + 1;
        }
        if file_index == almanac.len() {
            panic!("Map not found for map from {} to {}",self.source, self.destination);
        }
        file_index = file_index + 1;
        while file_index < almanac.len() && almanac[file_index] != "" {
            let range_description: Vec<usize> = almanac[file_index].split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
            self.add_range(range_description[1], range_description[2], range_description[0]);
            file_index = file_index + 1;
        }   
    }

    fn get_dest(&self, source_index: usize) -> usize {
        for (source, length, dest) in &self.map {
            if source_index >= *source && source_index < *source + *length {
                return dest + source_index - source;
            }
        }
        return source_index;
    }
}

#[test]
fn test_gardenMap() {

    let mut myMap = GardenMap::new("seed", "soil");

    myMap.add_range(98,2,50);
    myMap.add_range(50,48,52);
    assert_eq!(myMap.get_dest(0),0);
    assert_eq!(myMap.get_dest(1),1);
    assert_eq!(myMap.get_dest(30),30);
    assert_eq!(myMap.get_dest(23),23);
    assert_eq!(myMap.get_dest(49),49);
    assert_eq!(myMap.get_dest(50),52);
    assert_eq!(myMap.get_dest(51),53);
    assert_eq!(myMap.get_dest(52),54);
    assert_eq!(myMap.get_dest(97),99);
    assert_eq!(myMap.get_dest(98),50);
    assert_eq!(myMap.get_dest(99),51);
    assert_eq!(myMap.get_dest(100),100);
}


pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let (_, seeds_string) = line_array[0].split_at("seeds: ".len());
    let seed_list: Vec<usize> = seeds_string.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();

    let mut seed_soil_map = GardenMap::new("seed","soil");
    let mut soil_fertilizer_map = GardenMap::new("soil", "fertilizer");
    let mut fertilizer_water_map = GardenMap::new("fertilizer", "water");
    let mut water_light_map = GardenMap::new("water","light");
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

    let mut min_location = usize::MAX;
    for seed in seed_list {
        let soil = seed_soil_map.get_dest(seed);
        let fert = soil_fertilizer_map.get_dest(soil);
        let water = fertilizer_water_map.get_dest(fert);
        let light = water_light_map.get_dest(water);
        let temp = light_temp_map.get_dest(light);
        let hum = temp_humidity_map.get_dest(temp);
        let loc = humidity_location_map.get_dest(hum);
        println!("Seed {} -> Soil {} -> fert {} -> water {} -> light {} -> temp {} -> hum {} -> loc {}", seed, soil, fert, water, light, temp, hum, loc);
        min_location = min_location.min(loc);

    }
    println!("Min location is {}", min_location);

}
