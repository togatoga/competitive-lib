class Annealing {
public:
  const double TEMP_CTRL_PARAM = 4.8;
  double initial_temp, current_temp;
  double simulate_time;
  double start_time, end_time;
  double best_score;
  double param1, param2;
  XorShift rnd;
  Annealing(double initial_temp, double end_time, double start_time, XorShift rnd) {
    this->initial_temp = initial_temp;
    this->current_temp = initial_temp;
    this->rnd = rnd;

    this->best_score = 0;

    this->start_time = start_time;
    this->end_time = end_time;
    this->simulate_time = end_time - start_time;

    this->param1 = 1.0 / (simulate_time / TEMP_CTRL_PARAM);
    this->param2 = 1.0 / exp(TEMP_CTRL_PARAM);
  }
  bool cool_down(double elapsed) {
    if (elapsed >= end_time)
      return false;
    // linear function
    // current_temp = initial_temp + (final_temp - initial_temp) * ((time
    // -start_time) / simulate_time);
    // exp function
    current_temp = initial_temp * exp((end_time - elapsed) * param1) * param2;
    return true;
  }
  bool transition(int current_s, int neighbor_s) {
    // maximization problem
    if (current_s < neighbor_s)
      return true;
    double provability = exp((neighbor_s - current_s) / current_temp);
    // minimization problem
    // if (current_s > neighbor_s) return true;
    // double provability = exp((current_s - neighbor_s) / current_temp);

    if (provability >= rnd.sample())
      return true;
    return false;
  }
};
