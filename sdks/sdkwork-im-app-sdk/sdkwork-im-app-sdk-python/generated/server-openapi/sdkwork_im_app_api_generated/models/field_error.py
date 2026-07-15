from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class FieldError:
    field: str
    message: str
    code: Optional[int] = None
    i18n_key: Optional[str] = None
    params: Optional[Dict[str, str]] = None
