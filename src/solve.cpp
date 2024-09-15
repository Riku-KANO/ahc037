#pragma GCC target("avx2")
#pragma GCC optimize("Ofast")
#pragma GCC optimize("unroll-loops")
#include <iostream>
#include <fstream>
#include <sstream>
#include <istream>

#include <array>
#include <list>
#include <map>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include <string>
#include <tuple>
#include <bitset>
#include <queue>
#include <deque>
#include <complex>

#include <memory>
#include <algorithm>
#include <numeric>
#include <utility>
#include <functional>
#include <random>
#include <iomanip>
#include <chrono>
#include <thread>
#include <limits>

#include <ctime>
#include <cmath>
#include <cstdio>
#include <cstring>
#include <cassert>
using namespace std;
using ll = long long;
using u16 = uint16_t;
using u32 = uint32_t;
using u64 = uint64_t;
using i16 = int16_t;
using i32 = int32_t;
using i64 = int64_t;

using byte = uint8_t;

#define FOR(i, s, e) for (int i = s; i < e; i++)
#define Vec vector

template <typename T>
void chmin(T &x, T y) { x = min(x, y); }
template <typename T>
void chmax(T &x, T y) { x = max(x, y); }

std::random_device seed_gen;
std::mt19937 mt(seed_gen());
std::uniform_real_distribution<> rand01(0, 1);
std::uniform_real_distribution<> randReal(0, 1);

constexpr i32 INF = 1 << 30;
constexpr i64 LINF = 1LL << 60;
constexpr i64 MOD = 998244353;
constexpr double TL = 1.95;
const double PI = std::acos(-1);
constexpr i32 DI[4] = {0, 1, 0, -1};
constexpr i32 DJ[4] = {1, 0, -1, 0};
const std::string DIRS = "RDLU";
constexpr i32 DI8[8] = {0, 1, 1, 1, 0, -1, -1, -1};
constexpr i32 DJ8[8] = {1, 1, 0, -1, -1, -1, 0, 1};
inline double get_time(clock_t start_time)
{
    return (double)(clock() - start_time) / CLOCKS_PER_SEC;
}

inline bool is_out(int i, int j, int h, int w) { return i < 0 || j < 0 || i >= h || j >= w; }
inline bool is_out(int i, int j, int w) { return is_out(i, j, w, w); }
inline bool is_in(int i, int j, int h, int w) { return !is_out(i, j, h, w); }
inline bool is_in(int i, int j, int w) { return !is_out(i, j, w); }

template <typename T>
inline T pow2(T x) { return x * x; }

//-- globals --
clock_t start_time;
//-------------


void solve() {

}

int main(int argc, char *argv[])
{
    start_time = clock();
    solve();
    return 0;
}
