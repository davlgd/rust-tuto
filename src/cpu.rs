pub fn get_cpu_count() -> (usize, usize)
{
    (num_cpus::get(), num_cpus::get_physical())
}