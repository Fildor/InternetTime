using System.Globalization;

Console.WriteLine("@{0}", GetCurrentInternetTimeString("000.0"));

string GetCurrentInternetTimeString(string format)
{
    var utcNow = DateTimeOffset.UtcNow;
    double value = ((utcNow.ToUnixTimeSeconds() + 3600)%(24*3600)) / 86.4;
    return value.ToString(format, CultureInfo.InvariantCulture);
}
