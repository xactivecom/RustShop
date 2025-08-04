use crate::args::{
    UserSubcommand,
    UserCommand,
    CreateUser,
    UpdateUser,
    DeleteEntity,
};

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create(user) => {
            create_user(user);
        }
        UserSubcommand::Update(user) => {
            update_user(user);
        }
        UserSubcommand::Delete(entity) => {
            delete_user(entity);
        }
        UserSubcommand::Show => {
            show_users();
        }
    }
}

fn create_user(user: CreateUser) {
    println!("create_user {:?}", user);
}

fn update_user(user: UpdateUser) {
    println!("update_user {:?}", user);
}

fn delete_user(entity: DeleteEntity) {
    println!("delete_user {:?}", entity);
}

fn show_users() {
    println!("show_users");
}
