use types::linked_list;

const REPEAT_CNT: usize = 10;

#[test]
fn test_push_pop() {
    let mut memory = vec![0usize; 1024];

    let value1 = (&mut memory[0])  as *mut usize;
    let value2 = (&mut memory[8])  as *mut usize;
    let value3 = (&mut memory[16]) as *mut usize;
    let value4 = (&mut memory[24]) as *mut usize;

    let mut list = linked_list::List::new();
    let mut push_pop = || {
        list.push_front(value1);
        list.push_front(value2);
        list.push_front(value3);
        list.push_front(value4);

        assert_eq!(list.count(), 4);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(value1));
        assert_eq!(iter.next(), Some(value2));
        assert_eq!(iter.next(), Some(value3));
        assert_eq!(iter.next(), Some(value4));
        assert_eq!(iter.next(), None);

        assert_eq!(list.pop_front(), Some(value4));
        assert_eq!(list.pop_front(), Some(value3));
        assert_eq!(list.pop_front(), Some(value2));
        assert_eq!(list.pop_front(), Some(value1));
        assert_eq!(list.pop_front(), None);
    };

    for _ in 0..REPEAT_CNT {
        push_pop();
    }
}

#[test]
fn test_push_after_pop() {
    let mut memory = vec![0usize; 1024];

    let value1 = (&mut memory[0])  as *mut usize;
    let value2 = (&mut memory[8])  as *mut usize;

    let mut list = linked_list::List::new();
    let mut push_pop = || {
        list.push_front(value1);
        list.push_front(value2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(value1));
        assert_eq!(list.pop_front(), Some(value2));

        list.push_front(value2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(value1));
        assert_eq!(iter.next(), Some(value2));
        assert_eq!(iter.next(), None);

        assert_eq!(list.pop_front(), Some(value2));
        assert_eq!(list.pop_front(), Some(value1));
        assert_eq!(list.pop_front(), None);
    };

    for _ in 0..REPEAT_CNT {
        push_pop();
    }
}

#[test]
fn test_pop_empty() {
    let mut list = linked_list::List::new();
    let mut push_pop = || {
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
        assert_eq!(list.pop_front(), None);
    };

    for _ in 0..REPEAT_CNT {
        push_pop();
    }
}

#[test]
fn test_pop_head() {
    let mut memory = vec![0usize; 1024];

    let value1 = (&mut memory[0])  as *mut usize;
    let value2 = (&mut memory[8])  as *mut usize;

    let mut list = linked_list::List::new();
    let mut push_pop = || {
        list.push_front(value1);
        list.push_front(value2);

        let tmp = list.pop(value2);
        assert_eq!(tmp, Some(value2));

        assert_eq!(list.pop_front(), Some(value1));
        assert_eq!(list.pop_front(), None);
    };

    for _ in 0..REPEAT_CNT {
        push_pop();
    }
}

#[test]
fn test_pop_last() {
    let mut memory = vec![0usize; 1024];

    let value1 = (&mut memory[0])  as *mut usize;

    let mut list = linked_list::List::new();
    let mut push_pop = || {
        list.push_front(value1);

        let tmp = list.pop(value1);
        assert_eq!(tmp, Some(value1));

        assert_eq!(list.pop_front(), None);
    };

    for _ in 0..REPEAT_CNT {
        push_pop();
    }
}
