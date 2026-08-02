from setuptools import find_packages, setup

package_name = 'tf2_learning'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(),
    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name],
        ),
        (
            'share/' + package_name,
            ['package.xml'],
        ),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='jazib-manzoor',
    maintainer_email='your_email@example.com',
    description='Learning TF2 in ROS 2',
    license='TODO',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'tf_broadcaster = tf2_learning.tf_broadcaster:main',
            'tf_listener = tf2_learning.tf_listener:main',
            'marker_publisher = tf2_learning.marker_publisher:main',
        ],
    },
)
