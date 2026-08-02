from launch import LaunchDescription
from launch.actions import TimerAction, IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch_ros.actions import Node

from ament_index_python.packages import get_package_share_directory

import os
import xacro


def generate_launch_description():

    package_path = get_package_share_directory("robot_description")

    xacro_file = os.path.join(
        package_path,
        "urdf",
        "robot.xacro"
    )

    world_file = os.path.join(
        package_path,
        "worlds",
        "empty.world"
    )

    robot_description = xacro.process_file(xacro_file).toxml()

    return LaunchDescription([

        # Robot State Publisher
        Node(
            package="robot_state_publisher",
            executable="robot_state_publisher",
            parameters=[
                {"robot_description": robot_description}
            ],
            output="screen",
        ),

        # Launch Gazebo (Official ROS-Gazebo launcher)
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(
                os.path.join(
                    get_package_share_directory("ros_gz_sim"),
                    "launch",
                    "gz_sim.launch.py",
                )
            ),
            launch_arguments={
                "gz_args": world_file,
            }.items(),
        ),

        # Spawn Robot after Gazebo starts
        TimerAction(
            period=3.0,
            actions=[
                Node(
                    package="ros_gz_sim",
                    executable="create",
                    arguments=[
                        "-world", "default",
                        "-string", robot_description,
                        "-name", "roborack",
                        "-x", "0",
                        "-y", "0",
                        "-z", "0.3",
                    ],
                    output="screen",
                ),
            ],
        ),
    ])