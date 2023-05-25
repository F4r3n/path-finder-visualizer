#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{collections::{VecDeque, HashSet}, rc::Rc};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PathFindingInfo {
  list: Vec<i8>,
  size_x : usize,
  size_y : usize
}

fn find_arrival_from_array(list : &[i8]) -> Option<usize>
{
  list.iter().position(|&v|v == -2)
}

fn find_departure_from_array(list : &[i8]) -> Option<usize>
{
  list.iter().position(|&v|v == -1)
}

fn find_around(position : usize, size_x :usize, size_y : usize, around : & mut[Option<usize>;9])
{
  let mut i: usize = 0;
  let current_x: isize = (position%size_x) as isize;
  let current_y: isize = (position/size_y) as isize;

  for x in 0..3 {
    let pos_x: isize = (position%size_x + x) as isize - 1;

    for y in 0..3 {
      let pos_y: isize = (position/size_y + y) as isize - 1;

      //Prevent diagonal
      if (current_x - pos_x) != 0 && (current_y - pos_y) !=0
      {
        continue;
      }
      
      if pos_x >= 0 && pos_x < size_x as isize && pos_y >= 0 && pos_y < size_y as isize
      {
        let index: usize = (pos_x + (pos_y*(size_x as isize))) as usize;
        
        if index > 0 && index < (size_x*size_y) && index != position {
          around[i] = Some(index);
        }
      }
      i+=1;
    }
  }
}

#[derive(Clone, serde::Serialize)]
struct PathFindingCurrent {
  index: usize,
}

#[derive(Clone, serde::Serialize)]
struct PathFindingFound {
  list: Vec<usize>,
  path: Vec<usize>
}

struct Pair {
  from : Option<Rc<Pair>>,
  current : usize
}


#[tauri::command]
fn find_path( info : PathFindingInfo) -> Result<PathFindingFound, ()>{

  let departure = find_departure_from_array(&info.list[..]);
  let arrival = find_arrival_from_array(&info.list[..]);
  let mut found = false;

  let mut result : PathFindingFound = PathFindingFound { list: Vec::new(), path:Vec::new() };

  if let (Some(departure), Some(arrival)) = (departure, arrival) {
    let mut positions_done = HashSet::new();
    
    let mut queue : VecDeque<Rc<Pair>> = VecDeque::new();
    queue.push_back(Rc::new(Pair{ from: None, current:departure}));

    let mut around : [Option<usize>; 9] = [None;9];
    while !queue.is_empty() {
      if let Some(current_index) = queue.pop_front() {
        
        around.iter_mut().for_each(|x|*x = None);

        find_around( current_index.current, info.size_x, info.size_y, & mut around);
        for v in around {
          if let Some(v) = v {

            // value > 0 are considered as an obstacle
            if info.list[v] > 0 || positions_done.contains(&v) {
              continue;
            }
            
            result.list.push(v);
            queue.push_back(Rc::new(Pair{current:v, from:Some(Rc::clone(&current_index))}));

            if v == arrival {
              println!("arrived");

              found = true;
              break;
            }
            else {
              positions_done.insert(v);
            }
          }
        }

      }
      if found {
        break;
      }
    }

    if !queue.is_empty() {
      let mut it : &Rc<Pair> = queue.back().unwrap();

      loop {
          result.path.push(it.current);
          if it.from.is_some() {
              it = it.from.as_ref().unwrap();
          }
          else {
              break;
          }
    
      }
      result.path.reverse();

    }

  }

  Ok(result)
}


fn main() {

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![find_path])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
