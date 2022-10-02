add_executable(example_app main.cc)

message(STATUS
    "The project name = ${PROJECT_NAME}"
)

target_link_libraries(example_app ${PROJECT_NAME})