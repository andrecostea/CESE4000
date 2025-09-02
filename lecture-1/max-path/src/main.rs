const TRIANGLE: [[u32; 15]; 15] = [
    [75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [95, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [17, 47, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [18, 35, 87, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [20,  4, 82, 47, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [19,  1, 23, 75,  3, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [88,  2, 77, 73,  7, 63, 67, 0, 0, 0, 0, 0, 0, 0, 0],
    [99, 65,  4, 28,  6, 16, 70, 92, 0, 0, 0, 0, 0, 0, 0],
    [41, 41, 26, 56, 83, 40, 80, 70, 33, 0, 0, 0, 0, 0, 0],
    [41, 48, 72, 33, 47, 32, 37, 16, 94, 29, 0, 0, 0, 0, 0],
    [53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 0, 0, 0, 0],
    [70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, 0, 0, 0],
    [91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 0, 0],
    [63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 0],
    [ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23],
];

fn main() {
    // We’ll copy the triangle into a mutable 2D array so we can update it
    let mut dp = TRIANGLE;

    // Work from the second-to-last row upward
    let mut row = 13;
    while row >= 0 {
        let mut col = 0;
        while col <= row {
            let left = dp[row + 1][col];
            let right = dp[row + 1][col + 1];
            if left > right {
                dp[row][col] += left;
            } else {
                dp[row][col] += right;
            }
            col += 1;
        }
        if row == 0 {
            break; // avoid underflow since row is unsigned
        }
        row -= 1;
    }

    println!("The maximum total from top to bottom is {}.", dp[0][0]);
}
