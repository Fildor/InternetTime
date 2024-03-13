function getInternetTime ()
    mask = 24*3600
    bielTime = (os.time() + 3600) % mask
    result = bielTime / 86.4 
    return string.format("%3.1f", result);
end

print("@" .. getInternetTime());
