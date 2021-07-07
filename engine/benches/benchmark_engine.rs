use criterion::{criterion_group, criterion_main, Criterion};
use engine::{Board, Point, Snake};

fn benchmark_n_snakes(c: &mut Criterion, snakes: Vec<Snake>) {
    let n_snakes = snakes.len();

    c.bench_function(&format!("Engine_board_21x21_{}_snakes", n_snakes), |b| {
        let mut board = Board::new(21, 21, snakes.clone());
        b.iter(|| {
            for _ in 0..15 {
                if board.alive_snakes().count() > 0 {
                    board.step((0..n_snakes).into_iter().map(|_| rand::random()).collect());
                }
            }
        })
    });
}

pub fn benchmark_engine_10_snakes(c: &mut Criterion) {
    let snakes = [
        Point { x: 2, y: 4 },
        Point { x: 5, y: 1 },
        Point { x: 17, y: 12 },
        Point { x: 16, y: 12 },
        Point { x: 15, y: 12 },
        Point { x: 15, y: 11 },
        Point { x: 15, y: 10 },
        Point { x: 10, y: 19 },
        Point { x: 9, y: 19 },
        Point { x: 0, y: 19 },
    ]
    .iter()
    .map(|&p| Snake::new(p))
    .collect();

    benchmark_n_snakes(c, snakes);
}

pub fn benchmark_engine_4_snakes(c: &mut Criterion) {
    let snakes = [
        Point { x: 2, y: 4 },
        Point { x: 5, y: 1 },
        Point { x: 17, y: 12 },
        Point { x: 10, y: 19 },
    ]
    .iter()
    .map(|&p| Snake::new(p))
    .collect();

    benchmark_n_snakes(c, snakes);
}

criterion_group!(
    benches,
    benchmark_engine_4_snakes,
    benchmark_engine_10_snakes
);
criterion_main!(benches);
