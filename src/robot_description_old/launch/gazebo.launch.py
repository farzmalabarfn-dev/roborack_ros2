from launch import LaunchDescription
from launch.actions import (
    IncludeLaunchDescription,
    TimerAction,
    ExecuteProcess,
)
from launch.launch_description_sources import PythonLaunchDescriptionSource

from launch_ros.actions import Node

from ament_index_python.packages import get_package_share_directory

import os
import xacro


def generate_launch_description():

    pkg_share = get_package_share_directory("robot_description")

    # Robot xacro file
    xacro_file = os.path.join(
        pkg_share,
        "urdf",
        "robot.xacro"
    )

    # Gazebo world
    world_file = os.path.join(
        pkg_share,
        "worlds",
        "empty.world"
    )

    # Controller config
    config_file = os.path.join(
        pkg_share,
        "config",
        "controllers.yaml"
    )

    # Process xacro
    doc = xacro.process_file(
        xacro_file,
        mappings={
            "config_file": config_file
        }
    )

    robot_description = doc.toxml()

    return LaunchDescription([

        # Robot State Publisher
        Node(
            package="robot_state_publisher",
            executable="robot_state_publisher",
            output="screen",
            parameters=[
                {
                    "robot_description": robot_description,
                    "use_sim_time": True
                }
            ],
        ),

        # Start Gazebo
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(
                os.path.join(
                    get_package_share_directory("ros_gz_sim"),
                    "launch",
                    "gz_sim.launch.py"
                )
            ),
            launch_arguments={
                "gz_args": world_file + " -r"
            }.items(),
        ),

        # Gazebo Clock Bridge
        Node(
            package="ros_gz_bridge",
            executable="parameter_bridge",
            arguments=[
                "/clock@rosgraph_msgs/msg/Clock[gz.msgs.Clock"
            ],
            output="screen",
        ),

        # Spawn Robot
        TimerAction(
            period=3.0,
            actions=[
                Node(
                    package="ros_gz_sim",
                    executable="create",
                    output="screen",
                    arguments=[
                        "-topic",
                        "robot_description",
                        "-name",
                        "roborack",
                        "-x", "0",
                        "-y", "0",
                        "-z", "0.3",
                    ],
                )
            ],
        ),

        # Spawn Joint State Broadcaster
        TimerAction(
            period=15.0,
            actions=[
                ExecuteProcess(
                    cmd=[
                        "ros2",
                        "run",
                        "controller_manager",
                        "spawner",
                        "joint_state_broadcaster",
                        "-c",
                        "/controller_manager",
                        "-p",
                        config_file,
                        "--controller-manager-timeout",
                        "60",
                    ],
                    output="screen",
                )
            ],
        ),

        # Spawn Diff Drive Controller
        TimerAction(
            period=18.0,
            actions=[
                Node(
                    package="controller_manager",
                    executable="spawner",
                    arguments=[
                        "diff_drive_controller",
                        "-c",
                        "/controller_manager",
                        "-p",
                        config_file,
                    ],
                    output="screen",
                )
            ],
        ),

    ])