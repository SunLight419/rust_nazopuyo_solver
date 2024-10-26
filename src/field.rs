use std::collections::VecDeque;

pub trait Field {
    fn new() -> Self
    where
        Self: Sized;
    fn from_u8(value: [[u8; 13]; 6]) -> Self;
    fn from_char(value: [[char; 13]; 6]) -> Self;
    // fn from(v: [[u32; 13]; 6]) -> Self;
    fn set(&mut self, y: usize, x: usize, v: u8);
    fn get(&self, y: usize, x: usize) -> u8;
    // todo? field::fall() に切り出すべき？
    fn fall(&mut self);
    //fn chain(&mut self) -> u32;
    fn is_empty(&self) -> bool;
    fn is_alive(&self) -> bool;
}

/// 一番上にあるぷよのインデックス
/// None => 列が空
/// ✟伝家の宝刀 人力二分探索✟
/// TODO ぷよを置ける位置に変更した方がよい？
pub fn get_top<F: Field>(field: &F, x: usize) -> Option<usize> {
    if field.get(12, x) == 0 {
        return None;
    }
    if field.get(6, x) != 0 {
        if field.get(3, x) != 0 {
            if field.get(1, x) != 0 {
                if field.get(0, x) != 0 {
                    return Some(0);
                } else {
                    return Some(1);
                }
            } else {
                if field.get(2, x) != 0 {
                    return Some(2);
                } else {
                    return Some(3);
                }
            }
        } else {
            if field.get(4, x) != 0 {
                return Some(4);
            } else if field.get(5, x) != 0 {
                return Some(5);
            } else {
                return Some(6);
            }
        }
    }

    if field.get(9, x) != 0 {
        if field.get(7, x) != 0 {
            return Some(7);
        } else if field.get(8, x) != 0 {
            return Some(8);
        } else {
            return Some(9);
        }
    }

    if field.get(10, x) != 0 {
        return Some(10);
    }

    if field.get(11, x) != 0 {
        return Some(11);
    }

    Some(12)
}

pub fn chain<F: Field>(field: &mut F) -> u32
where
    F: Clone,
{
    let mut chain_count = 0u32;
    let dy = [0i32, 1, 0, -1];
    let dx = [1i32, 0, -1, 0];

    loop {
        let mut chained = false;
        let mut vis = [[false; 6]; 13];

        let mut next_field = field.clone();

        for i in 1..13 {
            for j in 0..6 {
                if vis[i][j] {
                    continue;
                }
                vis[i][j] = true;
                let color = next_field.get(i, j);
                if color <= 1 {
                    continue;
                }
                let mut que = VecDeque::new();
                let mut connected = VecDeque::new();

                que.push_back((i, j));
                connected.push_back((i, j));

                while !que.is_empty() {
                    let (y, x) = que.pop_front().unwrap();
                    for k in 0..4 {
                        let ny = y as i32 + dy[k];
                        let nx = x as i32 + dx[k];
                        if nx < 0
                            || ny < 0
                            || 6 <= nx
                            || 13 <= ny
                            || vis[ny as usize][nx as usize]
                            || ny == 0
                        {
                            continue;
                        }
                        let new_color = next_field.get(ny as usize, nx as usize);
                        if new_color != color || new_color == 0 || new_color == 1 {
                            continue;
                        }
                        vis[ny as usize][nx as usize] = true;
                        que.push_back((ny as usize, nx as usize));
                        connected.push_back((ny as usize, nx as usize));
                    }
                }

                if connected.len() >= 4 {
                    chained = true;
                    for (y, x) in connected {
                        next_field.set(y, x, 0);
                        for k in 0..4 {
                            let ny = y as i32 + dy[k];
                            let nx = x as i32 + dx[k];
                            if nx < 0
                                || ny < 0
                                || 6 <= nx
                                || 13 <= ny
                                || ny == 0
                                || next_field.get(ny as usize, nx as usize) != 1
                            {
                                continue;
                            }
                            next_field.set(ny as usize, nx as usize, 0);
                        }
                    }
                }
            }
        }

        if !chained {
            break;
        }
        chain_count += 1;
        *field = next_field;
        field.fall();
    }

    chain_count
}

pub fn has_chain<F: Field>(field: &F) -> bool {
    let dy = [0i32, 1, 0, -1];
    let dx = [1i32, 0, -1, 0];

    let mut vis = [[false; 6]; 13];

    for i in 1..13 {
        for j in 0..6 {
            if vis[i][j] {
                continue;
            }
            vis[i][j] = true;
            let color = field.get(i, j);
            if color == 0 || color == 1 {
                continue;
            }
            let mut que = VecDeque::new();
            let mut connected = VecDeque::new();

            que.push_back((i, j));
            connected.push_back((i, j));

            while !que.is_empty() {
                let (y, x) = que.pop_front().unwrap();
                for k in 0..4 {
                    let ny = y as i32 + dy[k];
                    let nx = x as i32 + dx[k];
                    if nx < 0
                        || ny < 0
                        || 6 <= nx
                        || 13 <= ny
                        || vis[ny as usize][nx as usize]
                        || ny == 0
                    {
                        continue;
                    }
                    let new_color = field.get(ny as usize, nx as usize);
                    if new_color != color || new_color == 0 || new_color == 1 {
                        continue;
                    }
                    vis[ny as usize][nx as usize] = true;
                    que.push_back((ny as usize, nx as usize));
                    connected.push_back((ny as usize, nx as usize));
                }
            }

            if connected.len() >= 4 {
                return true;
            }
        }
    }

    false
}


#[allow(unused)]
pub fn kenny_bench<T>() where T: Field + Clone {
    use crate::field;
    use std::time::Instant;
    let count = 1_000_000;
    let mut fields = Vec::with_capacity(count);
    for _ in 0..count {
        let kenny = [
            [0u8, 5, 6, 5, 6, 3, 6, 6, 6, 5, 6, 6, 6],
            [0u8, 4, 4, 4, 3, 4, 3, 3, 5, 3, 5, 5, 5],
            [5u8, 4, 5, 5, 4, 5, 4, 4, 5, 4, 3, 3, 3],
            [6u8, 5, 6, 6, 5, 6, 5, 5, 4, 6, 4, 4, 4],
            [3u8, 6, 3, 3, 6, 3, 6, 6, 5, 4, 6, 6, 6],
            [3u8, 4, 3, 4, 4, 4, 3, 3, 4, 4, 5, 5, 5],
        ];
        let field = T::from_u8(kenny);
        fields.push(field);

    }
    println!("{} loop kenny test: type of {}", count,  std::any::type_name::<T>());
    let t_start = Instant::now();

    for f in fields.iter_mut() {
        //f.chain();
        field::chain(f);
    }

    let t_end = Instant::now();
    let ell = t_end - t_start;
    println!("{:?} {:?}", ell, ell / count as u32);
}