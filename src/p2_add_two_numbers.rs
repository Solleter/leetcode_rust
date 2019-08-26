// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn initList() -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let input1 = vec![2, 4, 3];
    let input2 = vec![5, 6, 4];

    let mut header1: Option<Box<ListNode>> = None;
    let mut operator: &Option<Box<ListNode>> = &None;



    for val in input1 {
        let newNode = Some(Box::new(ListNode::new(val)));
        match header1 {
            None => {
                header1 = newNode;
                operator = &header1;
            },
            (_) => {
                let mut refLastNode = &(*operator).unwrap();
                (*refLastNode).next = newNode;
                operator = &(*refLastNode).next;
            }
        }
    }

    // for val in input1 {
    //     let node = Some(Box::new(ListNode::new(val)));
    //     if header1 == None {
    //         header1 = &node;
    //         operator = &node;
    //     }else{
    //         *operator.unwrap().next = node;
    //         operator = &node;
    //     }
    // }

    (None, None)
}




// pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     (Option(None), Option(None))
// }


// fn initList () -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
//     let input1 = vec![2, 4, 3];
//     let input2 = vec![5, 6, 4];

//     let mut header1 = None;
//     let mut header2 = None;
//     let mut operator = None;

//     for val in input1 {
//         let node = ListNode::new(val);
//         if header1 == None {
//             header1 = Some(Box::new(node));
//             operator = Some(node);
//         }else{
//             operator.next = Some(Box::new(node));
//             operator = Some(node);
//         }
//     }
    
    // let len1 = input1.len();
    // let len2 = input2.len();
    // let mut index = 0;

    // while index < len1 || index < len2 {
    //     if index < len1 {
    //         println!("Input1 {}: {}", index, input1[index]);
    //     }
    //     if index < len2 {
    //         println!("Input2 {}: {}", index, input2[index]);
    //     }
    //     index += 1;
    // }

    // for i in 0..len as usize {
    //     println!("{}", input1[i]);
    // }

//     (None, None)
// }


pub fn solution() {
    initList();


    
}