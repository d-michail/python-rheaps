"""Python bindings for the `rheaps` heap/priority-queue library."""

from .__version__ import __title__, __description__, __url__
from .__version__ import __version__, __backend_version__
from .__version__ import __author__, __author_email__, __license__
from .__version__ import __copyright__

from ._rheaps import *  # noqa: F401,F403
from ._rheaps import __all__  # noqa: F401
