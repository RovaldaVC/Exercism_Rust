use std::cmp::min;

#[derive(Debug)]
pub struct HighScores<'a>{
    scores_list:&'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self { 
        HighScores{
            scores_list : scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores_list
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores_list.last().copied()        
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_list.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three:Vec<u32> = Vec::new();
        let datas = self.scores_list.iter().copied();
        let mut whole_list:Vec<u32> = datas.collect();
        whole_list.sort_by(|a,b| b.cmp(a));

        let list_lenght = min(3, whole_list.len());
        for i in 0..list_lenght{
            top_three.push(whole_list[i])
        };
        top_three
    }
}


fn main() {}
