from math import ceil, floor

from . import solve


@solve(expected=633, resource="fraud_notifications.txt")
def num_notifications(data):
    [first_line, expenditure] = data.split("\n")
    _, days = first_line.split()
    expenditure, days = [int(e) for e in expenditure.split()], int(days)

    notifications = 0

    counts = [0] * 201
    for e in expenditure[:days]:
        counts[e] += 1

    def calculate_median(median_by):
        k = 0

        median = 0
        for i, n in enumerate(counts):
            k += n
            if k > median_by:
                median = i
                break

        return median

    def median_by(op):
        return op((days - 1) / 2)

    for i, e in enumerate(expenditure[days:], days):
        lo_med = calculate_median(median_by(floor))
        hi_med = calculate_median(median_by(ceil))

        if e >= lo_med + hi_med:
            notifications += 1

        counts[expenditure[i - days]] -= 1
        counts[e] += 1

    return notifications
