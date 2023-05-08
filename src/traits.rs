use std::fmt::Debug;

fn main() {
    let db1 = MySQL{};
    db1.open();
    db1.close();
    purge_db(&db1);

    let db2 = Postgres{};
    db2.open();
    db2.close();
    purge_db_v2(&db2);

    let mut dbx: &dyn DB = &db1;
    dbx.open();
    dbx.close();

    dbx = &db2;
    dbx.open();
    dbx.close();

    let _msqldb = create_mysql_db();

    let s1 = String::from("abcde");
    {
        let s2 = String::from("ab");
        let result = get_longer(s1.as_str(), s2.as_str());
        println!("{}", result);
    }
    let s3 = String::from("abx");
    {
        let s1 = String::from("abasdf");
        let ret;
        {
            let _s2 = String::from("abc");
            // ret = get_longer(s1.as_str(), s2.as_str()); // s2 should have same or longer lifetime as return
            ret = get_longer(s1.as_str(), s3.as_str());
        }
        println!("{}", ret)
    }

    let _x: &'static str = "abc"; // static lifetime, lives as long as the program
}

fn purge_db(db: &impl DB) {
    db.open();
    println!("purging DB");
    db.close();
}

fn purge_db_v2<T: DB>(db: &T) {
    db.open();
    println!("purging DB v2");
    db.close();
}

trait DB {
    fn open(&self) { // default implementation
        println!("try to open the DB");
    }
    fn close(&self);
}

struct MySQL {}
struct Postgres {}

impl DB for MySQL {
    fn open(&self) {
        println!("open mysql DB");
    }
    fn close(&self) {
        println!("close mysql DB");
    }
}

impl DB for Postgres {
    fn open(&self) {
        println!("open postgres DB");
    }
    fn close(&self) {
        println!("close postgres DB");
    }
}

fn _x_func<T: Debug + DB>(_t: &T) { // multiple trait bounds

}

fn _x_func_1<T>(_t: &T)
where
  T: Debug + DB
{ // syntax sugar

}

fn create_mysql_db() -> impl DB {
    return MySQL{};
}

// fn create_db(db_type: u8) -> impl DB { can only return a single type that implements DB trait
//     match db_type {
//         0 => return MySQL{},
//         1 => return Postgres{},
//         _ => panic!("unknown db type {}", db_type)
//     }
// }

fn get_longer<'a>(x: &'a str, y: &'a str) -> &'a str { // need to set lifetime for the return, so the compiler can verify if return will be valid
    return if x.len() > y.len() {
        x
    } else {
        y
    };
}