using System;
using System.Globalization;

namespace InternetTime
{
    class Program
    {
        static void Main(string[] args)
        {
            double internetTime = GetInternetTime();
            Console.WriteLine("@{0}", internetTime.ToString("000.0", CultureInfo.InvariantCulture));
        }

        private static double GetInternetTime()
        {
            DateTimeOffset now = DateTimeOffset.UtcNow;
            return ((now.ToUnixTimeSeconds() + 3600) % (24 * 3600)) / 86.4;
        }
    }
}
