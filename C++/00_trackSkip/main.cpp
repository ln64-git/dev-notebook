#include <iostream>
#include <cstdlib>
#include <string>
#include <chrono>
#include <thread>

// Function to simulate typing "/skip" and pressing Enter multiple times
void writeSkip(int count)
{
    for (int i = 0; i < count; ++i)
    {
        // Type "/skip", press Tab, and simulate pressing the Enter key
        std::system("echo 'type /skip' | dotool");
        std::system("echo 'type \nkey tab' | dotool");
        std::system("echo 'type \nkey enter' | dotool");
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
    // std::this_thread::sleep_for(std::chrono::seconds(3));

    // Call the function with the user's input
    writeSkip(count);
    return 0;
}
