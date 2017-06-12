//calculate table
template<typename T>
vector<int> make_table(const T& s) {
    int n = s.size();
    vector<int> ret(n+1);
    ret[0] = -1;
    int j = -1;
    for (int i = 0; i < n; i++) {
        while (j >= 0 && s[i] != s[j]) j = ret[j];
        ret[i+1] = ++j;
    }
    return ret;
}

// How many word contains in str
// ret's value that starts str[idx...idx+word.size()-1] == word
template<typename T>
vector<int> kmp(const T& str, const T& word) {
    vector<int> table = make_table(word), ret;
    int m = 0, i = 0, n = str.size();
    while (m+i < n) {
        if (word[i] == str[m+i]) {
            if (++i == (int)(word.size())) {
                ret.push_back(m);
                m = m+i-table[i];
                i = table[i];
            }
        } else {
            m = m+i-table[i];
            if (i > 0) i = table[i];
        }
    }
    return ret;
}
