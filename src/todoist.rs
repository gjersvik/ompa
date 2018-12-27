use todoist::{Client, ResourceType};

pub fn todoist(token: &str){
    let mut client = Client::new(token);
    let res = client.sync(&[ResourceType::Items]).unwrap();
    let items = res.items.unwrap();
    for item in items {
        println!("{}", item.content.unwrap_or_default())
    }
}