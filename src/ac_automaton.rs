use std::collections::VecDeque;
use std::convert::From;


const fn count_host_valid_character() -> usize{
// 'A-Za-z' "!$&'()*+,;=:%" "-._~" "0-9"
//    26   + 13 + 4 + 10 = 53
    26+13+4+10
}

#[derive(Copy,Clone)]
enum EdgeType{
    TrieEdge(usize),
    FailEdge(usize)
}

impl EdgeType{
    fn value(&self)->usize{
        match self{
            EdgeType::TrieEdge(v) => {*v}
            EdgeType::FailEdge(v) => {*v}
        }
    }
}

#[derive(Copy,Clone)]
pub enum MatchType{
    Domain(bool),
    SubStr(bool),
    Full(bool)
}

impl From<bool> for MatchType{
    fn from(v:bool)->Self{
        MatchType::Full(v)
    }
}

pub struct ACAutomaton{
    trie: Vec<[EdgeType;count_host_valid_character()]>,
    fail: Vec<usize>,
    exists: Vec<MatchType>,
    count: usize
}


impl ACAutomaton{
    pub fn new(size:usize)->ACAutomaton{
        ACAutomaton {
            trie:vec![[EdgeType::FailEdge(0);53];size],
            fail:vec![0;size],
            exists:vec![false.into();size],
            count: 0
        }
    }

    pub fn reverse_insert(&mut self, input_string:&str,match_type:MatchType){
        let mut node = 0;
        for c in input_string.chars().rev() {
            // new node
            let idx =char2idx(c);
            if self.trie[node][idx].value()==0 {
                self.count+=1;
                if self.trie.len()<self.count+1{
                    self.trie.push([EdgeType::FailEdge(0);53]);
                    self.fail.push(0);
                    self.exists.push(false.into());
                }
                self.trie[node][idx]=EdgeType::TrieEdge(self.count);
            }

            node =self.trie[node][idx].value();
        }
        self.exists[node]=match_type;
    }


    pub fn build(&mut self){
        let mut queue:VecDeque<EdgeType> = VecDeque::new();
        for i in 0..count_host_valid_character(){
            if self.trie[0][i].value()!=0 {
                queue.push_back(self.trie[0][i]);
            }
        }
        loop{
            match queue.pop_front(){
                None => {break}
                Some(node) => {
                    let node =node.value();
                    for i in 0..count_host_valid_character(){
                        if self.trie[node][i].value()!=0 {
                            self.fail[self.trie[node][i].value()]= self.trie[self.fail[node]][i].value();
                            queue.push_back(self.trie[node][i]);
                        } else {
                            self.trie[node][i]= match self.trie[self.fail[node]][i]{
                                EdgeType::TrieEdge(v) => {EdgeType::FailEdge(v)}
                                EdgeType::FailEdge(v) => {EdgeType::FailEdge(v)}
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn reverse_query(& self, query_string:&str)->bool{
        let mut node = 0;
        let mut full_match = true;
        // 1. the match string is all through trie edge. FULL MATCH
        // 2. the match string is through a fail edge. NOT FULL MATCH
        // 2.1 Through a fail edge, but there exists a valid node. DOMAIN or SUBSTR
        for (i,c) in query_string.chars().rev().enumerate(){
            node = match self.trie[node][char2idx(c)]{
                EdgeType::TrieEdge(v) => {v}
                EdgeType::FailEdge(v) => {
                    full_match &= false;
                    v
                }
            };
            let mut cur_node = node;
            while cur_node!=0 {
                match self.exists[node]{
                    MatchType::SubStr(v)=>{
                        if v{
                            return v;
                        }
                    }
                    MatchType::Domain(v)=>{
                        // look ahead
                        match query_string.chars().rev().nth(i+1){
                            None|Some('.') => {
                                if v{return v}
                            }
                            _ =>{}
                        }
                    }
                    _ => {}
                }
                cur_node = self.fail[cur_node];
            }
        }
        match self.exists[node]{
            MatchType::Full(v)=>{full_match&v}
            _=>{false}
        }
    }

}



fn char2idx(c:char)->usize{
    match c {
        'A'|'a'=>0,
        'B'|'b'=>1,
        'C'|'c'=>2,
        'D'|'d'=>3,
        'E'|'e'=>4,
        'F'|'f'=>5,
        'G'|'g'=>6,
        'H'|'h'=>7,
        'I'|'i'=>8,
        'J'|'j'=>9,
        'K'|'k'=>10,
        'L'|'l'=>11,
        'M'|'m'=>12,
        'N'|'n'=>13,
        'O'|'o'=>14,
        'P'|'p'=>15,
        'Q'|'q'=>16,
        'R'|'r'=>17,
        'S'|'s'=>18,
        'T'|'t'=>19,
        'U'|'u'=>20,
        'V'|'v'=>21,
        'W'|'w'=>22,
        'X'|'x'=>23,
        'Y'|'y'=>24,
        'Z'|'z'=>25,
        '!'=>26,
        '$'=>27,
        '&'=>28,
        '\''=>29,
        '('=>30,
        ')'=>31,
        '*'=>32,
        '+'=>33,
        ','=>34,
        ';'=>35,
        '='=>36,
        ':'=>37,
        '%'=>38,
        '-'=>39,
        '.'=>40,
        '_'=>41,
        '~'=>42,
        '0'=>43,
        '1'=>44,
        '2'=>45,
        '3'=>46,
        '4'=>47,
        '5'=>48,
        '6'=>49,
        '7'=>50,
        '8'=>51,
        '9'=>52,
        _ =>0
    }
}
