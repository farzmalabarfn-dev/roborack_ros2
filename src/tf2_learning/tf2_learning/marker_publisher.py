import rclpy
from rclpy.node import Node

from visualization_msgs.msg import Marker


class MarkerPublisher(Node):

    def __init__(self):
        super().__init__('marker_publisher')

        self.publisher = self.create_publisher(
            Marker,
            'visualization_marker',
            10
        )

        self.timer = self.create_timer(
            0.1,
            self.publish_marker
        )

    def publish_marker(self):

        marker = Marker()

        marker.header.frame_id = "base_link"
        marker.header.stamp = self.get_clock().now().to_msg()

        marker.ns = "robot"

        marker.id = 0

        marker.type = Marker.CUBE
        marker.action = Marker.ADD

        marker.pose.position.x = 0.0
        marker.pose.position.y = 0.0
        marker.pose.position.z = 0.0

        marker.pose.orientation.w = 1.0

        marker.scale.x = 0.4
        marker.scale.y = 0.3
        marker.scale.z = 0.2

        marker.color.a = 1.0
        marker.color.r = 0.0
        marker.color.g = 1.0
        marker.color.b = 0.0

        self.publisher.publish(marker)


def main(args=None):

    rclpy.init(args=args)

    node = MarkerPublisher()

    rclpy.spin(node)

    node.destroy_node()

    rclpy.shutdown()


if __name__ == "__main__":
    main()