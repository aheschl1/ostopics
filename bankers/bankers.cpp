#include<iostream>
#include<vector>
#include<map>
#include<unordered_map>
#include<string>
#include<algorithm>
#include<set>
#include<cmath>
#include<string>
#include<deque>
#include<unordered_set>
#include<queue>

typedef long long int ll;
typedef unsigned long long ull;

using namespace std;

bool is_safe(vector<vector<int>> &allocations, vector<vector<int>> &maxs, vector<int> &available){
    // compute the need matrix
    vector<vector<int>> needs;
    for(int i = 0; i < allocations.size(); i++){
        vector<int> this_need;
        vector<int> *this_alloc = &allocations[i];
        vector<int> *this_maxs = &maxs[i];
        // need is max - alloc
        for(int n = 0; n < this_alloc->size(); n++){
            this_need.push_back(this_maxs->at(n) - this_alloc->at(n));
        }
        needs.push_back(this_need);
    }
    // need matrix done.
    vector<bool> safe(allocations.size(), false); // represent if process i is safe
    while(find(safe.begin(), safe.end(), false) != safe.end()){
        bool stuck = true; // make sure we don't go through 2 full times
        for(int i = 0; i < allocations.size(); i++){
            // check if i is unsafe, and if need <= available
            if(safe[i]){continue;}
            // check need<=available
            bool is_safe = true;
            for(int n = 0; n < needs[i].size(); n++){
                is_safe = is_safe && needs[i][n] <= available[n];
            }
            if(!is_safe){continue;}
            // now update available with what is currently allocated, and set safe
            for(int n = 0; n < allocations[i].size(); n++){
                available[n] += allocations[i][n];
                allocations[i][n] = 0;
            }
            safe[i] = true;
            stuck = false;
            break;
        }
        if(stuck)
            break;
    }
    return find(safe.begin(), safe.end(), false) == safe.end();

}

void solve(){
    /**
     * Read the input file and then see if it is a safe state.
     */
    int n_resources, n_processes; cin >> n_resources >> n_processes;
    vector<vector<int>> allocations;
    vector<vector<int>> maxs;
    vector<int> available;
    // read avilable
    for(int i = 0; i < n_resources; i++){
        int a; cin >> a;
        available.push_back(a);
    }
    while(n_processes--){
        // read the allocation and max
        vector<int> this_alloc;
        vector<int> this_max;
        for(int i = 0; i < n_resources; i++){
            int alloc; cin >> alloc;
            this_alloc.push_back(alloc);
        }
        for(int i = 0; i < n_resources; i++){
            int maximum; cin >> maximum;
            this_max.push_back(maximum);
        }
        allocations.push_back(this_alloc);
        maxs.push_back(this_max);
    }
    bool safe = is_safe(allocations, maxs, available);
    cout << (safe ? "Safe" : "Unsafe") << endl;
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    solve();
    return 0;
}