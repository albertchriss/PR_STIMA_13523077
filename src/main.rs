use std::io::{self, BufRead};

const INF: i32 = i32::MAX / 2;

fn solve_tsp(graph: &Vec<Vec<i32>>) -> Option<(i32, Vec<usize>)> {
    let n = graph.len();

    if n == 0 {
        return None;
    }
    if n == 1 {
        let cost = if graph[0][0] == INF { 0 } else { graph[0][0] };
        return Some((cost, vec![0, 0]));
    }

    let start_node: usize = 0;
    let mut dp = vec![vec![INF; n]; 1 << n];
    let mut parent = vec![vec![0; n]; 1 << n];

    dp[1 << start_node][start_node] = 0;

    for mask in 1..(1 << n) {
        for u in 0..n {
            if (mask >> u) & 1 == 1 { // Jika kota u ada di mask
                if dp[mask][u] == INF { 
                    continue;
                }
                // Untuk setiap kota 'v' yang belum ada di 'mask' sebagai kota berikutnya
                for v in 0..n {
                    if (mask >> v) & 1 == 0 { // Jika kota v belum ada di mask
                        if graph[u][v] != INF { // Jika ada jalur dari u ke v
                            let new_mask = mask | (1 << v);
                            let new_cost = dp[mask][u] + graph[u][v];

                            if new_cost < dp[new_mask][v] {
                                dp[new_mask][v] = new_cost;
                                parent[new_mask][v] = u; // u adalah parent dari v dalam jalur ini
                            }
                        }
                    }
                }
            }
        }
    }

    let final_mask = (1 << n) - 1; // Mask ketika semua kota telah dikunjungi
    let mut min_tour_cost = INF;
    let mut last_node_in_tour = 0; // Kota terakhir sebelum kembali ke start_node

    for u in 0..n {
        if dp[final_mask][u] != INF && graph[u][start_node] != INF {
            let current_total_cost = dp[final_mask][u] + graph[u][start_node];
            if current_total_cost < min_tour_cost {
                min_tour_cost = current_total_cost;
                last_node_in_tour = u;
            }
        }
    }

    if min_tour_cost == INF {
        return None; // Tidak ada tur yang valid ditemukan
    }

    // Rekonstruksi jalur
    let mut tour = Vec::with_capacity(n + 1);
    let mut current_mask = final_mask;
    let mut current_node = last_node_in_tour;

    let mut reconstructed_path_segment = vec![0; n];
    for i in (0..n).rev() {
        reconstructed_path_segment[i] = current_node;
        if i == 0 {
            assert_eq!(current_node, start_node, "Kesalahan rekonstruksi jalur: tidak berakhir di start_node");
            break;
        }
        let prev_node = parent[current_mask][current_node];
        current_mask ^= 1 << current_node; // Hapus current_node dari mask
        current_node = prev_node;
    }
    
    tour.extend_from_slice(&reconstructed_path_segment);
    tour.push(start_node); // Tambahkan start_node di akhir untuk melengkapi siklus

    Some((min_tour_cost, tour))
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Membaca N dan M
    let first_line = match iterator.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Gagal membaca baris pertama (N dan M).");
            return;
        }
    };
    let mut parts = first_line.split_whitespace();
    let n: usize = match parts.next().unwrap_or("").parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Format N tidak valid.");
            return;
        }
    };
    let m: usize = match parts.next().unwrap_or("").parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Format M tidak valid.");
            return;
        }
    };

    if n == 0 {
        println!("\nSolusi TSP Tidak Dapat Diproses:");
        println!("------------------------------------");
        println!("Jumlah kota (N) adalah 0. Tidak ada tur yang mungkin.");
        println!("------------------------------------");
        return;
    }
    
    let mut graph = vec![vec![INF; n]; n];

    // Membaca M edge
    for i in 0..m {
        let line = match iterator.next() {
            Some(Ok(l)) => l,
            _ => {
                eprintln!("Gagal membaca definisi edge ke-{}", i + 1);
                return;
            }
        };
        let mut edge_parts = line.split_whitespace();
        let u: usize = match edge_parts.next().unwrap_or("").parse::<usize>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Format u pada edge ke-{} tidak valid.", i + 1);
                return;
            }
        };
        let v: usize = match edge_parts.next().unwrap_or("").parse::<usize>() {
             Ok(val) => val,
            Err(_) => {
                eprintln!("Format v pada edge ke-{} tidak valid.", i + 1);
                return;
            }
        };
        let w: i32 = match edge_parts.next().unwrap_or("").parse::<i32>() {
             Ok(val) => val,
            Err(_) => {
                eprintln!("Format w pada edge ke-{} tidak valid.", i + 1);
                return;
            }
        };

        if u > 0 && u <= n && v > 0 && v <= n {
            graph[u - 1][v - 1] = std::cmp::min(graph[u - 1][v - 1], w);
            graph[v - 1][u - 1] = std::cmp::min(graph[v - 1][u - 1], w);
        } else {
            eprintln!("Indeks node tidak valid pada edge ke-{}: {} atau {}. Node harus antara 1 dan {}.", i + 1, u, v, n);
            return;
        }
    }

    // Menyelesaikan TSP dan mencetak output
    match solve_tsp(&graph) {
        Some((cost, path)) => {
            println!("\nSolusi TSP Ditemukan:");
            println!("------------------------------------");
            println!("Biaya Minimum Tur : {}", cost);
            print!("Jalur Optimal     : ");
            for (idx, &node_val) in path.iter().enumerate() {
                print!("{}", node_val + 1); // Konversi kembali ke 1-indexed untuk output
                if idx < path.len() - 1 {
                    print!(" -> ");
                }
            }
            println!();
            println!("------------------------------------");
        }
        None => {
            println!("\nSolusi TSP Tidak Ditemukan.");
            println!("------------------------------------");
            println!("Pastikan graf memungkinkan untuk mengunjungi semua kota dan kembali ke kota awal.");
            println!("Kemungkinan penyebab:");
            println!("1. Graf tidak terhubung dengan baik.");
            println!("2. Tidak ada jalur kembali ke kota awal.");
            println!("------------------------------------");
        }
    }
}
