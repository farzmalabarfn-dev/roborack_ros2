import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts


class AddTwoNumbersServer(Node):

    def __init__(self):
        # Initialize the ROS 2 node with the name "add_two_numbers_server"
        super().__init__('add_two_numbers_server')

        # Create a service named "add_two_ints"
        # Service Type: AddTwoInts
        # Callback Function: add_two_numbers_callback
        self.srv = self.create_service(
            AddTwoInts,
            'add_two_ints',
            self.add_two_numbers_callback
        )

        self.get_logger().info('Add Two Ints Service is Ready!')

    def add_two_numbers_callback(self, request, response):
        # Add the two integers received from the client
        response.sum = request.a + request.b

        # Print the received request in the terminal
        self.get_logger().info(
            f'Request Received: {request.a} + {request.b}'
        )

        # Return the response to the client
        return response


def main(args=None):
    # Initialize the ROS 2 Python client library
    rclpy.init(args=args)

    # Create the service node
    node = AddTwoNumbersServer()

    # Keep the node running and waiting for service requests
    rclpy.spin(node)

    # Clean up when the node is stopped
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()