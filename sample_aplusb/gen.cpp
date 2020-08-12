#include <fstream>

using namespace std;

void generate_file(string filename, int a, int b) {
  ofstream ofs(filename);
  ofs << a << " " << b << endl;
}
int main(void) {
  for (int i = 0; i < 10; i++) {
    generate_file(string("in/") + to_string(i) + ".txt", 100 * i, 20 * i + 1);
  }
}