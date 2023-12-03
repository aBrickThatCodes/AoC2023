// TODO: This could be better. Will I improve it? Maybe

namespace day3;

internal static class Program
{
    private static void Main() {
        var lines = File.ReadAllLines("input");
        Console.WriteLine($"Part 1: {Part1(lines)}");
        Console.WriteLine($"Part 2: {Part2(lines)}");
    }

    #region Part 1

    private static bool IsSchematicSymbol(this char c) {
        return !(char.IsDigit(c) || c == '.');
    }

    private static int Part1(string[] lines) {
        var lineLength = lines[0].Length;
        var sum = 0;
        for (var i = 0; i < lines.Length; i++) {
            var beginNumber = -1;
            var leftmost = 0;
            var currentLine = lines[i];
            for (var j = 0; j < lineLength; j++) {
                var currentIsDigit = char.IsDigit(currentLine[j]);
                var lineEnd = j == lineLength - 1;

                if (currentIsDigit)
                    if (beginNumber == -1) {
                        beginNumber = j;
                        leftmost = Math.Max(0, j - 1);
                    }

                if ((!currentIsDigit || lineEnd) && beginNumber != -1) {
                    var endNumber = currentIsDigit ? j : j - 1;
                    var isPart = false;

                    if (i > 0) {
                        var lineAbove = lines[i - 1];
                        for (var k = leftmost; k <= j; k++)
                            if (IsSchematicSymbol(lineAbove[k])) {
                                isPart = true;
                                break;
                            }
                    }

                    if (!isPart && i < lines.Length - 1) {
                        var lineBelow = lines[i + 1];
                        for (var k = leftmost; k <= j; k++)
                            if (IsSchematicSymbol(lineBelow[k])) {
                                isPart = true;
                                break;
                            }
                    }

                    if (!isPart && leftmost != beginNumber) isPart = IsSchematicSymbol(currentLine[leftmost]);

                    if (!isPart && endNumber != j) isPart = IsSchematicSymbol(currentLine[j]);

                    if (isPart) {
                        var s = currentLine.Substring(beginNumber, endNumber - beginNumber + 1);
                        var n = int.Parse(s);
                        sum += n;
                    }

                    beginNumber = -1;
                }
            }
        }

        return sum;
    }

    #endregion

    #region Part 2

    private class Gear
    {
        public int Degree = 1;
        public int Ratio;

        public Gear(int n) {
            Ratio = n;
        }
    }

    private static void AddToGear(this Dictionary<(int, int), Gear> gears, (int, int) key, int n) {
        if (!gears.TryGetValue(key, out var gear)) {
            gear = new Gear(n);
            gears.Add(key, gear);
        }
        else {
            gear.Degree++;
            if (gear.Degree < 3) gear.Ratio *= n;
            gears[key] = gear;
        }
    }

    private static int Part2(string[] lines) {
        var lineLength = lines[0].Length;
        var gears = new Dictionary<(int, int), Gear>();

        for (var i = 0; i < lines.Length; i++) {
            var beginNumber = -1;
            var leftmost = 0;
            var currentLine = lines[i];
            for (var j = 0; j < lineLength; j++) {
                var currentIsDigit = char.IsDigit(currentLine[j]);
                var lineEnd = j == lineLength - 1;

                if (currentIsDigit)
                    if (beginNumber == -1) {
                        beginNumber = j;
                        leftmost = Math.Max(0, j - 1);
                    }

                if ((!currentIsDigit || lineEnd) && beginNumber != -1) {
                    var endNumber = currentIsDigit ? j : j - 1;
                    var s = currentLine.Substring(beginNumber, endNumber - beginNumber + 1);
                    var n = int.Parse(s);

                    if (i > 0) {
                        var lineAbove = lines[i - 1];
                        for (var k = leftmost; k <= j; k++)
                            if (lineAbove[k] == '*')
                                gears.AddToGear((i - 1, k), n);
                    }

                    if (i < lines.Length - 1) {
                        var lineBelow = lines[i + 1];
                        for (var k = leftmost; k <= j; k++)
                            if (lineBelow[k] == '*')
                                gears.AddToGear((i + 1, k), n);
                    }

                    if (leftmost != beginNumber)
                        if (currentLine[leftmost] == '*')
                            gears.AddToGear((i, leftmost), n);

                    if (endNumber != j)
                        if (currentLine[j] == '*')
                            gears.AddToGear((i, j), n);
                    beginNumber = -1;
                }
            }
        }

        var gearRatios = from gear in gears.Values where gear.Degree == 2 select gear.Ratio;
        return gearRatios.Sum();
    }

    #endregion
}