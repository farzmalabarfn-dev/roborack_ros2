from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    return LaunchDescription([

        Node(
            package='my_first_robot',
            executable='action_server',
            name='action_server'
        ),

        Node(
            package='my_first_robot',
            executable='action_client',
            name='action_client'
        )
    ])