import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient

from my_robot_interfaces.action import CountUntil


class CountUntilActionClient(Node):

    def __init__(self):
        super().__init__('count_until_action_client')

        self._action_client = ActionClient(
            self,
            CountUntil,
            'count_until'
        )

        self.get_logger().info("Action Client Started!")
        self.declare_parameter('target', 10)
        self.target = self.get_parameter('target').value

    def send_goal(self, target):

        goal = CountUntil.Goal()
        goal.target_count = target

        self._action_client.wait_for_server()

        self.get_logger().info(f"Sending Goal: {target}")

        self._send_goal_future = self._action_client.send_goal_async(
            goal,
            feedback_callback=self.feedback_callback
        )

        self._send_goal_future.add_done_callback(
            self.goal_response_callback
        )

    def goal_response_callback(self, future):

        goal_handle = future.result()

        if not goal_handle.accepted:
            self.get_logger().info("Goal Rejected!")
            return

        self.get_logger().info("Goal Accepted!")

        self._get_result_future = goal_handle.get_result_async()

        self._get_result_future.add_done_callback(
            self.result_callback
        )

    def feedback_callback(self, feedback_msg):

        feedback = feedback_msg.feedback

        self.get_logger().info(
            f"Current Number: {feedback.current_number}"
        )

    def result_callback(self, future):

        result = future.result().result

        if result.success:
            self.get_logger().info("Action Completed Successfully!")
        else:
            self.get_logger().info("Action Failed!")

        rclpy.shutdown()


def main(args=None):

    rclpy.init(args=args)

    node = CountUntilActionClient()

    node.send_goal(node.target)

    rclpy.spin(node)


if __name__ == '__main__':
    main()