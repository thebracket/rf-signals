use rf_signal_algorithms::*;

fn main() {
    println!("Workspace root.");

    println!("{}", free_space_path_loss_db(5800.0, 10000.0));
}