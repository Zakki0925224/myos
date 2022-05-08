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
        return (convert_bytes_to_kb(bytes), "KB");
    }
    else if bytes < 1024 * 1024 * 1024
    {
        return (convert_bytes_to_mb(bytes), "MB");
    }
    else if bytes < 1024 * 1024 * 1024 * 1024
    {
        return (convert_bytes_to_gb(bytes), "GB");
    }
    else
    {
        return (convert_bytes_to_tb(bytes), "TB");
    }
}