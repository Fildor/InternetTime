function getInternetTime ()
    mask = 24*3600
    utcTime = (os.time() + 3600) % mask
    result = utcTime / 86.4 
    return string.format("%3.1f", result);
end

print("@" .. getInternetTime());