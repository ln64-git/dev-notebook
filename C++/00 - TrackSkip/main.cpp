#include <iostream>
#include <cstdlib>
#include <string>
// #include <thread>
// #include <chrono>

// Function to simulate typing "/skip" and pressing Enter multiple times
void writeSkip(int count)
{
    for (int i = 0; i < count; ++i)
    {
        // Type "/skip"
        std::system("ydotool type '/skip'");

        // Simulate pressing the Enter key
        std::system("ydotool key 28"); // 28 is the keycode for Enter
    }
}

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        std::cerr << "Usage: " << argv[0] << " <count>" << std::endl;
        return 1;
    }

    int count = std::stoi(argv[1]);

    // Wait a bit to allow the user to focus the text box
    // std::cout << "You have 5 seconds to focus the text box..." << std::endl;
    // std::this_thread::sleep_for(std::chrono::seconds(5));

    // Call the function with the user's input
    writeSkip(count);

    return 0;
}
