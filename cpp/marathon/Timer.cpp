//get millsec
class Timer {
public:
  Timer() {}
  void start() { start_time = get_mill_sec(); }
  //millsec
  double get_duration() { return get_mill_sec() - start_time; }
  void set_time_limit(double limit){time_limit = limit;}
  bool is_time_limit_exceeded(){return get_duration() > time_limit;}
private:
  double start_time;
  double time_limit;
  double get_mill_sec() {
    struct timeval res;
    gettimeofday(&res, NULL);
    return res.tv_sec * 1000 + res.tv_usec / 1000;
  }
};
