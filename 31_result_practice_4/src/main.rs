// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#[derive(Debug)]
enum EmployeeType {
    MaintenanceCrews,
    MarketingDepartmentEmployees,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}

#[derive(Debug)]
struct Employee {
    employee_type: EmployeeType,
    is_employed: bool,
}

fn verify_employment(employee: &Employee) -> Result<(), &'static str> {
    if employee.is_employed {
        Ok(())
    } else {
        Err("Employee is terminated and cannot access the building.")
    }
}

fn is_access_allowed_type(et: &EmployeeType) -> bool {
    matches!(et, EmployeeType::MaintenanceCrews | EmployeeType::MarketingDepartmentEmployees | EmployeeType::Managers)
}

fn can_enter_building(employee: &Employee) -> Result<(), &'static str> {
    verify_employment(employee)?;
    if is_access_allowed_type(&employee.employee_type) {
        Ok(())
    } else {
        Err("Employee position does not allow access to the building.")
    }
}

fn main() {
    // Example: A current manager (should access)
    let employee1 = Employee {
        employee_type: EmployeeType::Managers,
        is_employed: true,
    };
    match can_enter_building(&employee1) {
        Ok(_) => println!("The employee may access the building."),
        Err(e) => println!("The employee may not access the building: {}", e),
    }

    // Example: A terminated manager (should not access)
    let employee2 = Employee {
        employee_type: EmployeeType::Managers,
        is_employed: false,
    };
    match can_enter_building(&employee2) {
        Ok(_) => println!("The employee may access the building."),
        Err(e) => println!("The employee may not access the building: {}", e),
    }

    // Example: A current kitchen staff (should not access)
    let employee3 = Employee {
        employee_type: EmployeeType::KitchenStaff,
        is_employed: true,
    };
    match can_enter_building(&employee3) {
        Ok(_) => println!("The employee may access the building."),
        Err(e) => println!("The employee may not access the building: {}", e),
    }
}