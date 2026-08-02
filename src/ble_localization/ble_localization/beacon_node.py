import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Point


class BeaconNode(Node):

    def __init__(self):
        super().__init__('beacon_node')

        # Publisher for Beacon 1
        self.beacon1_pub = self.create_publisher(
            Point,
            '/beacon1',
            10
        )

        # Publisher for Beacon 2
        self.beacon2_pub = self.create_publisher(
            Point,
            '/beacon2',
            10
        )

        # Publisher for Beacon 3
        self.beacon3_pub = self.create_publisher(
            Point,
            '/beacon3',
            10
        )

        # Publisher for Beacon 4
        self.beacon4_pub = self.create_publisher(
            Point,
            '/beacon4',
            10
        )

        # Publish every second
        self.timer = self.create_timer(
            1.0,
            self.publish_beacons
        )

        self.get_logger().info("Beacon Node Started")

    def publish_beacons(self):

        b1 = Point()
        b1.x = 0.0
        b1.y = 0.0
        b1.z = 0.0

        b2 = Point()
        b2.x = 10.0
        b2.y = 0.0
        b2.z = 0.0

        b3 = Point()
        b3.x = 0.0
        b3.y = 10.0
        b3.z = 0.0

        b4 = Point()
        b4.x = 10.0
        b4.y = 10.0
        b4.z = 0.0

        self.beacon1_pub.publish(b1)
        self.beacon2_pub.publish(b2)
        self.beacon3_pub.publish(b3)
        self.beacon4_pub.publish(b4)

        self.get_logger().info("Published Beacon Positions")


def main(args=None):

    rclpy.init(args=args)

    node = BeaconNode()

    rclpy.spin(node)

    node.destroy_node()

    rclpy.shutdown()


if __name__ == '__main__':
    main()