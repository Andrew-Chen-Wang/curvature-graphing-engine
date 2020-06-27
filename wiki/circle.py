from argparse import ArgumentParser
from math import sqrt


def print_circle(max_radius: int, inner_radius: int, char_inner, char_outer, xscale):
    """
    Creates a circle with an inner and outer layer.
    The radius is the radius of the entire circle.
    The inner radius is meant to create the buffer
    area or the "outside" portion of the circle
    by suggesting the inside is an inner circle.

    Credit for help on getting this function:
    https://stackoverflow.com/a/33173243
    """
    # Maximum diameter
    width = int(0.5 + xscale * max_radius)

    def calculate_circle_width(rad, current_circle_rad) -> int:
        """Calculates printed width of circle"""
        return int(0.5 + xscale * sqrt(rad - current_circle_rad ** 2))

    printed_rad: int = max_radius ** 2
    printed_inner_rad: int = inner_radius ** 2
    print(printed_rad, printed_inner_rad)
    # TODO Show the top + bottom half of circle
    for y in range(-max_radius, max_radius + 1):
        # Find width at this height
        x = calculate_circle_width(printed_rad, y)
        # Create a list of all the characters necessary
        # for input.

        s = ""
        for i in range(-x // 4, x // 4):
            try:
                if abs(i) < xscale * sqrt(printed_inner_rad - i ** 2):
                    s += f"{char_inner} "
                else:
                    s += f"{char_outer} "
            except ValueError:
                s += f"{char_inner} "

        # Print the line and center it.
        print(s.center(width))


if __name__ == "__main__":
    ap = ArgumentParser()
    ap.add_argument(
        "-r",
        "--max-radius",
        type=int,
        required=True,
        help="Specify the radius of the entire circle.",
    )
    ap.add_argument(
        "-ir",
        "--inner-radius",
        type=int,
        required=False,
        default=None,
        help="The circle's inner radius.",
    ),
    ap.add_argument(
        "--inner-char",
        default="■",
        required=False,
        help="The character for the inner part of the circle",
    )
    ap.add_argument(
        "--outer-char",
        default="●",
        required=False,
        help="The character for the outer part of the circle",
    )
    ap.add_argument(
        "--xscale",
        default=5.1,
        type=float,
        required=False,
        help="Specify the scale factor of the printed circle. "
        "It may need to be fine tuned based on your screen",
    )

    args = vars(ap.parse_args())
    if args["inner_radius"] is not None:
        assert (
            args["max_radius"] >= args["inner_radius"]
        ), "The radius must be >= the inner radius."
    else:
        args["inner_radius"] = args["max_radius"]
    print_circle(
        max_radius=args["max_radius"],
        inner_radius=args["inner_radius"],
        char_inner=args["inner_char"],
        char_outer=args["outer_char"],
        xscale=args["xscale"],
    )
