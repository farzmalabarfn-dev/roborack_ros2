import rclpy
from rclpy.node import Node
from std_msgs.msg import String


class PublisherNode(Node):

    def __init__(self):
        super().__init__("publisher_node")

        # Create a publisher that publishes String messages
        # on the "robot_status" topic.
        self.publisher_ = self.create_publisher(
            String,
            "robot_status",
            10
        )

        # Create a timer that calls publish_status()
        # every 1 second.
        self.timer = self.create_timer(
            1.0,
            self.publish_status
        )

        # Counter for our messages.
        self.count = 0

    def publish_status(self):
        # Create an empty String message.
        msg = String()

        # Put text inside the message.
        msg.data = f"Robot Status: Running {self.count}"

        # Publish the message.
        self.publisher_.publish(msg)

        # Print the message in the terminal.
        self.get_logger().info(msg.data)

        # Increase the counter.
        self.count += 1


def main(args=None):
    rclpy.init(args=args)

    node = PublisherNode()

    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()