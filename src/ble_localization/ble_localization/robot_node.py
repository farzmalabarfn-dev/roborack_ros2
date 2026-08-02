import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Point


class RobotNode(Node):

    def __init__(self):
        super().__init__('robot_node')

        # Publisher
        self.publisher_ = self.create_publisher(
            Point,
            '/robot_pose',
            10
        )

        # Initial robot position
        self.x = 2.0
        self.y = 3.0

        # Timer (calls publish_position every second)
        self.timer = self.create_timer(
            1.0,
            self.publish_position
        )

        self.get_logger().info("Robot Node Started")

    def publish_position(self):

        # Move robot along the x-axis
        self.x += 0.2

        # Create message
        msg = Point()

        msg.x = self.x
        msg.y = self.y
        msg.z = 0.0

        # Publish
        self.publisher_.publish(msg)

        # Print position
        self.get_logger().info(
            f"Robot Position: ({msg.x:.2f}, {msg.y:.2f})"
        )


def main(args=None):

    rclpy.init(args=args)

    node = RobotNode()

    rclpy.spin(node)

    node.destroy_node()

    rclpy.shutdown()


if __name__ == '__main__':
    main()