pub fn convert_bytes_to_kb(bytes: u64) -> u64
{
    return bytes / 1024;
}

pub fn convert_bytes_to_mb(bytes: u64) -> u64
{
    return convert_bytes_to_kb(bytes) / 1024;
}

pub fn convert_bytes_to_gb(bytes: u64) -> u64
{
    return convert_bytes_to_mb(bytes) / 1024;
}

pub fn convert_bytes_to_tb(bytes: u64) -> u64
{
    return convert_bytes_to_gb(bytes) / 1024;
}

pub fn convert_bytes_to_any(bytes: u64) -> (u64, &'static str)
{
    if bytes < 1024
    {
        return (bytes, "B");
    }
    else if bytes < 1024 * 1024
    {
        return (convert_bytes_to_kb(bytes), "KiB");
    }
    else if bytes < 1024 * 1024 * 1024
    {
        return (convert_bytes_to_mb(bytes), "MiB");
    }
    else if bytes < 1024 * 1024 * 1024 * 1024
    {
        return (convert_bytes_to_gb(bytes), "GiB");
    }
    else
    {
        return (convert_bytes_to_tb(bytes), "TiB");
    }
}