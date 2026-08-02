import time

import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer

from my_robot_interfaces.action import CountUntil


class CountUntilActionServer(Node):

    def __init__(self):
        super().__init__('count_until_action_server')

        self._action_server = ActionServer(
            self,
            CountUntil,
            'count_until',
            self.execute_callback
        )

        self.get_logger().info("Count Until Action Server Started!")

    def execute_callback(self, goal_handle):

        self.get_logger().info("Goal received!")

        target = goal_handle.request.target_count

        feedback = CountUntil.Feedback()

        for i in range(1, target + 1):

            feedback.current_number = i

            goal_handle.publish_feedback(feedback)

            self.get_logger().info(f"Current Number: {i}")

            time.sleep(1)

        goal_handle.succeed()

        result = CountUntil.Result()
        result.success = True

        self.get_logger().info("Goal Completed!")

        return result


def main(args=None):
    rclpy.init(args=args)

    node = CountUntilActionServer()

    rclpy.spin(node)

    rclpy.shutdown()


if __name__ == '__main__':
    main()