use std::{collections::HashMap, io};

pub fn company_employee(){
    let mut company: HashMap<String, Vec<String>>= HashMap::new();


    // company.insert(String::from("DPS"),vec![String::from("lenisha")]);
    // let Some(x) = company.get_mut("DPS") else { todo!() };

    // x.push(String::from("Khulal"));

    // println!("{x:?}");

    // println!("{company:?}");

    println!("You will now enter the employee name along with thier department.");

    println!("How many employees would you like to add?");

    let mut employee_count= String::new();

    io::stdin()
        .read_line(&mut employee_count)
        .expect("Please enter the number of employees to add");
    
    let employee_count: i32 = employee_count.trim().parse().expect("Pleae enter a number");
    add_employee(employee_count,&mut company);

    println!("{company:?}");

    retrive_all_employee_name(&company);

    let mut department_to_retrive= String::new();

    println!("Enter the name of department whose employee you want to see listed");

    io::stdin()
        .read_line(&mut department_to_retrive)
        .expect("enter the department name");

    retrive_depart_employee_name(&company, department_to_retrive);

}

fn add_employee(employee_count:i32, company: &mut HashMap<String, Vec<String>>)-> &mut HashMap<String, Vec<String>>{
    for i in 0..employee_count{

        let mut department_name=String::new();
        let mut employee_name= String::new();



        println!("Enter the department");

        io::stdin()
            .read_line(&mut department_name)
            .expect("Please enter the number of employees to add");
        
        println!("Enter the name of employee");

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Enter the employee name");


        if let Some(employee_list_of_this_depart) = company.get_mut(&department_name.trim().to_string()){
            employee_list_of_this_depart.push(employee_name.trim().to_string());
        } else {
            company.insert(department_name.trim().to_string(),vec![employee_name.trim().to_string()]);
        };

    }

        println!("{company:?}");
        company
}

fn retrive_all_employee_name(company: & HashMap<String,Vec<String>>)->Vec<String>{
    let mut all_employee_names: Vec<String>=Vec::new();

    for (_,employee_names) in company{
        for employee_name in employee_names{
            all_employee_names.push(employee_name.to_string());
        }
    }

    all_employee_names.sort_by(|a, b| a.cmp(b));

    println!("{all_employee_names:?}");
    all_employee_names
}

fn retrive_depart_employee_name(company: & HashMap<String, Vec<String>>,department:String)->Option<Vec<String>>{

    if let Some(depart_employee_names)=company.get(&department.trim().to_string()){
        println!("employee names successfully retrived");

        let mut sorted_names: Vec<String>= depart_employee_names.clone();
        sorted_names.sort();
        println!("{sorted_names:?}");
        Some(sorted_names)
    }
    else{
        println!("The department is not included");
        None
    }

}