pub mod schema;
pub mod functions;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Env, String, Address, Vec};
use crate::schema::{Course, CourseModule};

#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {
    pub fn create_course(
        env: Env,
        title: String,
        description: String,
    ) -> Course {
        functions::create_course::course_registry_create_course(env, title, description)
    }

    pub fn get_course(env: Env, course_id: String) -> Course {
        functions::get_course::course_registry_get_course(&env, course_id)
    }

    pub fn get_courses_by_instructor(env: Env, instructor: Address) -> Vec<Course> {
        functions::get_courses_by_instructor::course_registry_get_courses_by_instructor(&env, instructor)
    }

    pub fn remove_module(env: Env, module_id: String) -> () {
        functions::remove_module::course_registry_remove_module(&env, module_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn add_module(
        env: Env,
        course_id: String,
        position: u32,
        title: String,
    ) -> CourseModule {
        functions::add_module::course_registry_add_module(env, course_id, position, title)
    }

    pub fn delete_course(env: Env, course_id: String) -> () {
        functions::delete_course::course_registry_delete_course(&env, course_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
