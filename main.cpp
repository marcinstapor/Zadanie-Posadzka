#include <iostream>
#include <cmath>
#include <algorithm>
#include <set>
#include <vector>
#include <stack>

using namespace std;



int main() {

   ios_base::sync_with_stdio(0);

     char a, b, c;
    while(!cin.eof())
    {
      cin >> a >> b;
	  if(cin.fail()) break;
	  c=(a-'0')*10+b-'0';
	 // if(c<' ') c+=100;
	  cout << c;
	}

   return 0;
}


