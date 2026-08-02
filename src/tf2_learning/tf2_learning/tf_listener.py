import rclpy

from rclpy.node import Node

from tf2_ros import Buffer
from tf2_ros import TransformListener
from tf2_ros import TransformException


class TFListener(Node):

    def __init__(self):

        super().__init__('tf_listener')

        self.buffer = Buffer()

        self.listener = TransformListener(self.buffer, self)

        self.timer = self.create_timer(1.0, self.lookup_transform)

    def lookup_transform(self):

        try:

            transform = self.buffer.lookup_transform(
                'world',
                'base_link',
                rclpy.time.Time()
            )

            x = transform.transform.translation.x
            y = transform.transform.translation.y
            z = transform.transform.translation.z

            self.get_logger().info(
                f"Robot Position: x={x}, y={y}, z={z}"
            )

        except TransformException as ex:

            self.get_logger().warn(str(ex))


def main(args=None):

    rclpy.init(args=args)

    node = TFListener()

    rclpy.spin(node)

    rclpy.shutdown()


if __name__ == '__main__':
    main()