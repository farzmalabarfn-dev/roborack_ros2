import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts


class AddTwoNumbersClient(Node):

    def __init__(self):
        super().__init__('add_two_numbers_client')

        self.client = self.create_client(
            AddTwoInts,
            'add_two_ints'
        )

        while not self.client.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('Waiting for the service...')

        request = AddTwoInts.Request()
        request.a = 25
        request.b = 75

        future = self.client.call_async(request)

        rclpy.spin_until_future_complete(self, future)

        response = future.result()

        self.get_logger().info(
            f'The sum is: {response.sum}'
        )


def main(args=None):
    rclpy.init(args=args)

    node = AddTwoNumbersClient()

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()